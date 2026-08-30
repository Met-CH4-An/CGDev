// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::rc::Rc;
use crate::backend::backend::Backend;
use crate::token::{Token, TokenType};
use crate::chunk_mask::ChunkMask;
use crate::chunk_mask_register::ChunkMaskRegister;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
enum TokenizerState {
    TAG_BEGIN_FIND,
    TAG_BEGIN_READING,
    TAG_END_READING,
    TAG_NAME_FIND,
    TAG_NAME_READING,
    TAG_ATTRIBUTE_NAME_FINE,
    TAG_ATTRIBUTE_NAME_READING,
    TAG_ATTRIBUTE_VALUE_FINE,
    TAG_ATTRIBUTE_VALUE_READING,
    END,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Tokenizer<TBackend>
where
TBackend: Backend {
    /// Регистры для построения масок.
    /// Registers for constructing masks.
    register_preset: ChunkMaskRegister<TBackend>,
    /// Данные для обработки.
    /// Data to be processed.
    data_ptr: *const u8,
    data_length: usize,
    data_rc: Rc<Vec<u8>>,
    /// Текущее состояние токенайзера.
    /// Current state of the tokenizer.
    state : TokenizerState,
    /// Текущее смещение внутри данных.
    /// Current offset within the data.
    current_in_data_position: usize,
    /// Текущее смещение внутри чанка.
    /// Current offset within the chunk.
    current_in_chunk_position: usize,
    /// Текущий чанк масок.
    /// Current chunk of masks.
    current_chunk_mask: ChunkMask<TBackend>,
    /// Ожидающий токен. Токен, который был найден при поиске другого.
    /// Pending token. A token that was found while searching for another.
    pending_token: Token,
    last_r_chevron: usize,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> Tokenizer<TBackend>
where
TBackend: Backend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            register_preset: unsafe {TBackend::buildChunkMaskRegister()},
            data_ptr: std::ptr::null(),
            data_length: 0,
            data_rc: Rc::<Vec<u8>>::new(Vec::<u8>::new()),
            state: TokenizerState::TAG_BEGIN_FIND,
            current_in_data_position: 0,
            current_in_chunk_position: 0,
            current_chunk_mask: ChunkMask::<TBackend>::s_create(),
            pending_token: Token::s_createEmpty(),
            last_r_chevron: 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_createWithData(data_rc: Rc<Vec<u8>>) -> Self {
        Self {
            register_preset: unsafe {TBackend::buildChunkMaskRegister()},
            data_ptr: data_rc.as_ptr(),
            data_length: 0,
            data_rc: data_rc,
            state: TokenizerState::TAG_BEGIN_FIND,
            current_in_data_position: 0,
            current_in_chunk_position: 0,
            current_chunk_mask: ChunkMask::<TBackend>::s_create(),
            pending_token: Token::s_createEmpty(),
            last_r_chevron: 0,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> Tokenizer<TBackend>
where
TBackend: Backend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Установка новых данных. Установка приводит к полному сбросу состояния токенайзера.
    /// Installing new data. This causes a complete reset of the tokenizer state.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn setData(&mut self, data_rc: Rc<Vec<u8>>) {
        self.reset();

        self.data_rc = data_rc;
        self.data_ptr = self.data_rc.as_ptr();
        self.data_length = self.data_rc.len();
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Сброс текущего состояния токенайзера. Токенайзер приводится в начальное состояние.
    /// Resets the current state of the tokenizer. The tokenizer is returned to its initial state.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn reset(&mut self) {
        // Строим первый чанк.
        // Building the first chunk.
        //let mut chunk_ = TokenizerChunkMask::<TBackend>::s_create();
        //unsafe { TBackend::buildChunk(&mut chunk_, self.data_ptr); }

        self.state = TokenizerState::TAG_BEGIN_FIND;
        self.current_in_data_position = 0;
        self.current_in_chunk_position = 0;
        self.current_chunk_mask = ChunkMask::<TBackend>::s_create();
        self.pending_token = Token::s_createEmpty();
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn nextToken1(&mut self) -> Token {
        let token_ = self.processState();

        token_
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



macro_rules! FIND_VALID_TZ {
    //($self: ident, $token_type: ident, $token_data_rng: ident,  $chunk_loop: lifetime, $analyze_loop: lifetime) => {{
    ($self: ident, $valid_mask: expr, $analyze_loop: lifetime) => {{
        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
        if $self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
            break $analyze_loop;
            //$self.nextChunk();
        }

        // Очищаем валидную и не валидную маски до найденной позиции.
        // Clear valid and invalid masks to the found position.
        $valid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);

        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
        // If the masks are empty, stop the analysis to get a new chunk.
        if $valid_mask == TBackend::ZERO {
            break $analyze_loop;
            //$self.nextChunk();
        }

        let valid_tz_ = TBackend::trailingZeros($valid_mask);

        // Сохраняем позицию.
        // Save the position.
        $self.current_in_chunk_position = valid_tz_ as usize + 1;

        valid_tz_
    }};
}

macro_rules! FIND_VALID_TZ_WITH {
    ($self: ident, $valid_mask: ident, $analyze_loop: lifetime, $invalid_mask: ident) => {{
        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
        if $self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
            break $analyze_loop;
            //$self.nextChunk();
        }

        // Очищаем валидную и не валидную маски до найденной позиции.
        // Clear valid and invalid masks to the found position.
        $valid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);
        $invalid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);

        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
        // If the masks are empty, stop the analysis to get a new chunk.
        if $valid_mask | $invalid_mask == TBackend::ZERO {
            break $analyze_loop;
            //$self.nextChunk();
        }

        let valid_tz_ = TBackend::trailingZeros($valid_mask);
        let invalid_tz_ = TBackend::trailingZeros($invalid_mask);

        //println!("{} {}", valid_tz_, invalid_tz_);

        // Если недопустимый бит встретился раньше, ошибка токенайзера.
        // If an invalid bit was encountered earlier, a tokenizer error occurs.
        if invalid_tz_ <= valid_tz_ {
            $self.state = TokenizerState::END;
            continue $analyze_loop;
        }

        // Сохраняем позицию.
        // Save the position.
        $self.current_in_chunk_position = valid_tz_ as usize + 1;

        valid_tz_
    }};
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные методы.
/// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> Tokenizer<TBackend>
where
TBackend: Backend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn processState(&mut self) -> Token {
        let mut b: usize = 0;

        let token_ = 'chunk: loop {
            'analyze: loop {
                match self.state {
                    // Поиск <.
                    // Search <.
                    TokenizerState::TAG_BEGIN_FIND => {
                        // Позиция: <'тут'.
                        // Position: <'here'.
                        let valid_tz_ = FIND_VALID_TZ!(self, self.current_chunk_mask.l_chevron_mask, 'analyze);

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_BEGIN_READING;

                        // Если между новой позицией в данных и старой больше одного, формируем новый токен как TokenType::TEXT.
                        // If there is more than one between the new data position and the old one, create a new token as TokenType::TEXT.
                        if self.current_in_data_position + valid_tz_ as usize - self.last_r_chevron > 2 {
                            let begin_ = self.last_r_chevron;
                            let end_ = self.current_in_data_position + valid_tz_ as usize - 1;
                            
                            let token_= Token::s_create(TokenType::TEXT, begin_ ..= end_);
                            
                            break 'chunk token_;
                        }
                    } // TokenizerState::TAG_BEGIN_FIND

                    // Анализ: <'тут'.
                    // Analysis: <'here'.
                    TokenizerState::TAG_BEGIN_READING => {
                        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            self.nextChunk();
                        }

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_NAME_FIND;

                        // begin_ = предыдущий <.
                        // begin_ = previous <.
                        let global_position_ = self.current_in_data_position + self.current_in_chunk_position;
                        let begin_ = global_position_ - 1;
                        let end_ = global_position_;

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << self.current_in_chunk_position;

                        // Если <'тут' == <'Aa-Zz'.
                        // If <'here' == <'Aa-Zz'.
                        let token_ = if bit_ & self.current_chunk_mask.letters_digitals_mask != TBackend::ZERO {
                            let end_ = begin_;

                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            let token_ = Token::s_create(TokenType::TAG_BEGIN, begin_..= end_);

                            token_
                        }

                        // Если <'тут' == <'/'.
                        // If <'here' == <'/'.
                        else if bit_ & self.current_chunk_mask.forward_slash != TBackend::ZERO {
                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            let token_ = Token::s_create(TokenType::TAG_BEGIN_CLOSE, begin_..= end_);

                            token_
                        }

                        // Если <'тут' == <'?'.
                        // If <'here' == <'?'.
                        else if bit_ & self.current_chunk_mask.question_mark != TBackend::ZERO {
                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            let token_ = Token::s_create(TokenType::TAG_BEGIN_INSTRUCTION, begin_..= end_);

                            token_
                        }

                        // Если любой другой символ <'тут' - невалидный формат.
                        // If any other character <'here' - invalid format.
                        else {
                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::END;

                            //self.data_ptr[..2].iter();

                            //let line = self.data_ptr[..2]
                            //    .iter()
                            //    .filter(|&&byte| byte == b'\n')
                            //    .count() + 1;

                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            let token_ = Token::s_create(TokenType::INVALID, begin_ ..= end_);

                            token_
                        };

                        break 'chunk token_;
                    } // TokenizerState::TAG_BEGIN_READING

                    TokenizerState::TAG_NAME_FIND => {
                        let mut valid_mask_ = self.current_chunk_mask.letters_digitals_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark;

                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;

                        let valid_tz_ = FIND_VALID_TZ_WITH!(
                            self,
                            valid_mask_,
                            'analyze,
                            invalid_mask_);

                        b = self.current_in_data_position + valid_tz_ as usize;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_NAME_READING;
                    } // TokenizerState::TAG_NAME_FIND

                    TokenizerState::TAG_NAME_READING => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark
                            | self.current_chunk_mask.separators_mask;

                        // Маска для невалидных значений.
                        // Mask for invalid values.
                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;

                        let valid_tz_ = FIND_VALID_TZ_WITH!(
                            self,
                            valid_mask_,
                            'analyze,
                            invalid_mask_);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        let global_position_ = self.current_in_data_position + valid_tz_ as usize - 1;
                        let begin_ = b;
                        let end_ = global_position_;

                        let token_ = Token::s_create(TokenType::TAG_NAME, begin_..= end_);

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << valid_tz_ as usize;

                        // Если на текущей позиции находится ' '.
                        // If the current position is ' '.
                        if bit_ & self.current_chunk_mask.separators_mask != TBackend::ZERO {
                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_ATTRIBUTE_NAME_FINE;
                        }

                        // Если на текущей позиции находится '>' или '/' или '?'.
                        // If the current position is '>' or '/' or '?'.
                        else {
                            self.current_in_chunk_position -= 1;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_END_READING;
                        }

                        break 'chunk token_;
                    } // TokenizerState::TAG_NAME_READING

                    TokenizerState::TAG_END_READING => {
                        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            self.nextChunk();
                        }

                        let global_position_ = self.current_in_data_position + self.current_in_chunk_position;
                        let begin_ = global_position_;
                        let end_ = global_position_;

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << self.current_in_chunk_position;

                        let token_ = if bit_ & self.current_chunk_mask.r_chevron_mask != TBackend::ZERO {
                            let token_ = Token::s_create(TokenType::TAG_END, begin_ ..= end_);

                            token_
                        }

                        else {
                            let token_ = if bit_ & self.current_chunk_mask.forward_slash != TBackend::ZERO {
                                self.current_in_chunk_position += 1;

                                // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                                // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                                if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                                    self.nextChunk();

                                    // устанавливаем бит на позицию, которую нужно найти
                                    //bit_ = TBackend::ONE;
                                }

                                // устанавливаем бит на позицию, которую нужно найти
                                let bit_ = TBackend::ONE << self.current_in_chunk_position;

                                let token_ = if bit_ & self.current_chunk_mask.r_chevron_mask != TBackend::ZERO {
                                    let token_ = Token::s_create(TokenType::TAG_END_CLOSE, begin_ ..= end_ + 1);

                                    token_
                                }

                                else {
                                    let token_ = Token::s_create(TokenType::INVALID, 0 ..= 0);

                                    token_
                                };

                                token_
                            }

                            else if bit_ & self.current_chunk_mask.question_mark != TBackend::ZERO {
                                self.current_in_chunk_position += 1;

                                // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                                // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                                if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                                    self.nextChunk();

                                    // устанавливаем бит на позицию, которую нужно найти
                                    //bit_ = TBackend::ONE;
                                }

                                // устанавливаем бит на позицию, которую нужно найти
                                let bit_ = TBackend::ONE << self.current_in_chunk_position;

                                let token_ = if bit_ & self.current_chunk_mask.r_chevron_mask != TBackend::ZERO {
                                    let token_ = Token::s_create(TokenType::TAG_END_INSTRUCTION, begin_ ..= end_ + 1);

                                    token_
                                }

                                else {
                                    let token_ = Token::s_create(TokenType::INVALID, 0 ..= 0);

                                    token_
                                };

                                token_
                            }

                            else {
                                let token_ = Token::s_create(TokenType::INVALID, 0 ..= 0);

                                token_
                            };

                            token_
                        };

                        self.last_r_chevron = token_.data_rng.end() + 1;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_BEGIN_FIND;

                        break 'chunk token_;
                    }

                    TokenizerState::TAG_ATTRIBUTE_NAME_FINE => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.letters_digitals_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark;

                        // Маска для невалидных значений.
                        // Mask for invalid values.
                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;

                        let valid_tz_ = FIND_VALID_TZ_WITH!(
                            self,
                            valid_mask_,
                            'analyze,
                            invalid_mask_);

                        b = self.current_in_data_position + valid_tz_ as usize;

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << valid_tz_ as usize;

                        // Если на текущей позиции находится символ.
                        // If there is a character at the current position.
                        if bit_ & self.current_chunk_mask.letters_digitals_mask != TBackend::ZERO {
                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_ATTRIBUTE_NAME_READING;
                        }

                        // Если на текущей позиции находится '>' или '/' или '?'.
                        // If the current position is '>' or '/' or '?'.
                        else {
                            self.current_in_chunk_position -= 1;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_END_READING;
                        }
                    }

                    TokenizerState::TAG_ATTRIBUTE_NAME_READING => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.separators_mask
                            | self.current_chunk_mask.equal_mask;

                        // Маска для невалидных значений.
                        // Mask for invalid values.
                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.question_mark
                            | self.current_chunk_mask.exclamation_mark
                            | self.current_chunk_mask.quote_mask;

                        let valid_tz_ = FIND_VALID_TZ_WITH!(
                            self,
                            valid_mask_,
                            'analyze,
                            invalid_mask_);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        let global_position_ = self.current_in_data_position + valid_tz_ as usize - 1;
                        let begin_ = b;
                        let end_ = global_position_;

                        let token_ = Token::s_create(TokenType::ATTRIBUTE_NAME, begin_..= end_);

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_VALUE_FINE;

                        break 'chunk token_;
                    }

                    TokenizerState::TAG_ATTRIBUTE_VALUE_FINE => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.quote_mask;

                        // Маска для невалидных значений.
                        // Mask for invalid values.
                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.question_mark
                            | self.current_chunk_mask.exclamation_mark
                            | self.current_chunk_mask.letters_digitals_mask
                            | self.current_chunk_mask.forward_slash;

                        let valid_tz_ = FIND_VALID_TZ_WITH!(
                            self,
                            valid_mask_,
                            'analyze,
                            invalid_mask_);

                        b = self.current_in_data_position + valid_tz_ as usize + 1;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_VALUE_READING;

                        continue 'analyze;
                    }

                    TokenizerState::TAG_ATTRIBUTE_VALUE_READING => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.quote_mask;

                        let valid_tz_ = FIND_VALID_TZ!(
                            self,
                            valid_mask_,
                           'analyze);

                        /*// Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            //break $analyze_loop;
                            //self.nextChunk();
                        }

                        // Очищаем валидную и не валидную маски до найденной позиции.
                        // Clear valid and invalid masks to the found position.
                        valid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);

                        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
                        // If the masks are empty, stop the analysis to get a new chunk.
                        if valid_mask_ == TBackend::ZERO {
                            break 'analyze;
                            //$self.nextChunk();
                        }

                        let valid_tz_ = TBackend::trailingZeros(valid_mask_);

                        // Сохраняем позицию.
                        // Save the position.
                        self.current_in_chunk_position = valid_tz_ as usize + 1;*/

                        //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[self.current_in_data_position ..= self.current_in_data_position + self.current_in_chunk_position]) };

                        //println!("{}", value_str_);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        let global_position_ = self.current_in_data_position + valid_tz_ as usize - 1;
                        let begin_ = b;
                        let end_ = global_position_;

                        let token_ = Token::s_create(TokenType::ATTRIBUTE_VALUE, begin_..= end_);

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_NAME_FINE;

                        break 'chunk token_;
                    }

                    TokenizerState::END => {
                        let token_ = Token::s_create(TokenType::INVALID, 0 ..= 0);

                        break 'chunk token_;
                    }

                    //_ => {}
                }
            } // 'analyze: loop

            self.nextChunk();
        }; // 'chunk: loop

        //let token_ = Token::s_create(token_type_, token_data_begin_..= token_data_end_);

        token_
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn nextChunk(&mut self) {
        self.current_in_data_position += TBackend::CHUNK_SIZE;
        self.current_in_chunk_position = 0;

        // Проверка, что работаем в диапазоне с данными.
        // Check that we are working in a range with data.
        if self.current_in_data_position <= self.data_length {
            let data_cptr_ = unsafe { self.data_ptr.add(self.current_in_data_position) };

            // Строим чанк через бэкенд. Бэкенд определяется трейтами и реализуется
            unsafe { TBackend::buildChunk(&mut self.register_preset, &mut self.current_chunk_mask, data_cptr_) };
        } else {
            self.state = TokenizerState::END
        }
    }
}

/*// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[repr(u32)]
enum MaskType {
    L_CHEVRON = 0,
    R_CHEVRON = 1,
    EQUAL,
    QUOTES,
    TAB_SPACE,
    LETTERS,
    SPECIAL,
}

impl<T> std::ops::Index<MaskType> for [T] {
    type Output = T;

    fn index(&self, index: MaskType) -> &Self::Output {
        return &self[index as usize];
    }
}

impl<T> std::ops::IndexMut<MaskType> for [T] {
    fn index_mut(&mut self, index: MaskType) -> &mut Self::Output {
        return &mut self[index as usize];
    }
}*/

/*// Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_chunk_mask.r_chevron_mask;

                        // Очищаем валидную и не валидную маски до найденной позиции.
                        // Clear valid and invalid masks to the found position.
                        //self.current_chunk_mask.l_chevron_mask &= !((1u32 << self.current_in_chunk_position) - 1u32);
                        valid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);

                        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
                        // If the masks are empty, stop the analysis to get a new chunk.
                        if valid_mask_ == TBackend::ZERO {
                            break 'analyze;
                        }

                        let valid_tz_ = TBackend::trailingZeros(valid_mask_);

                        // сохраняем позицию
                        self.current_in_chunk_position = valid_tz_ as usize;*/

/*// Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            break 'analyze;
                        }

                        // Очищаем валидную маску до найденной позиции.
                        // Clear the valid mask to the found position.
                        //self.current_chunk_mask.l_chevron_mask &= !((1u32 << self.current_in_chunk_position) - 1u32);
                        self.current_chunk_mask.l_chevron_mask &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);

                        // Если маска пустая, останавливаем анализ, чтобы получить новый чанк.
                        // If the mask is empty, stop the analysis to get a new chunk.
                        if self.current_chunk_mask.l_chevron_mask == TBackend::ZERO {
                            break 'analyze;
                        }

                        // Считаем количество нулей до первой единицы.
                        // Count the number of zeros before the first one.
                        let l_chevron_tz_: usize = TBackend::trailingZeros(self.current_chunk_mask.l_chevron_mask) as usize;*/

/*// Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_chunk_mask.letters_digitals_mask;

                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;


                        // Очищаем валидную и не валидную маски до найденной позиции.
                        // Clear valid and invalid masks to the found position.
                        //self.current_chunk_mask.l_chevron_mask &= !((1u32 << self.current_in_chunk_position) - 1u32);
                        valid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);
                        invalid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);

                        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
                        // If the masks are empty, stop the analysis to get a new chunk.
                        if valid_mask_ | invalid_mask_ == TBackend::ZERO {
                            break 'analyze;
                        }

                        let valid_tz_ = TBackend::trailingZeros(valid_mask_);
                        let invalid_tz_ = TBackend::trailingZeros(invalid_mask_);

                        // Если недопустимый бит встретился раньше, ошибка токенайзера.
                        // If an invalid bit was encountered earlier, a tokenizer error occurs.
                        if invalid_tz_ <= valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }*/

/*// Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark
                            | self.current_chunk_mask.separators_mask;

                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;


                        // Очищаем валидную и не валидную маски до найденной позиции.
                        // Clear valid and invalid masks to the found position.
                        //self.current_chunk_mask.l_chevron_mask &= !((1u32 << self.current_in_chunk_position) - 1u32);
                        valid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);
                        invalid_mask_ &= !((TBackend::ONE << self.current_in_chunk_position) - TBackend::ONE);

                        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
                        // If the masks are empty, stop the analysis to get a new chunk.
                        if valid_mask_ | invalid_mask_ == TBackend::ZERO {
                            break 'analyze;
                        }

                        let valid_tz_ = TBackend::trailingZeros(valid_mask_);
                        let invalid_tz_ = TBackend::trailingZeros(invalid_mask_);

                        // Если недопустимый бит встретился раньше, ошибка токенайзера.
                        // If an invalid bit was encountered earlier, a tokenizer error occurs.
                        if invalid_tz_ <= valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }*/