// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::{Deref, Range};
use std::rc::Rc;
use crate::token::{Token, TokenType};
use crate::tokenizer::tokenizer_chunk_mask::TokenizerChunkMask;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct AVX2 {}
pub struct AVX512 {}

mod private {
    pub trait TokenizerBackendPrivate {}
}

pub trait TokenizerBackend: private::TokenizerBackendPrivate {
    type CHUNK_TYPE1: Default + Copy
    + Into<u64>
    + From<u32>
    + PartialEq
    + std::ops::BitAnd<Output = Self::CHUNK_TYPE1>
    + std::ops::BitAndAssign
    + std::ops::BitOr<Output = Self::CHUNK_TYPE1>
    + std::ops::BitOrAssign
    + std::ops::BitXor<Output = Self::CHUNK_TYPE1>
    + std::ops::Not<Output = Self::CHUNK_TYPE1>
    + std::ops::Shl<usize, Output = Self::CHUNK_TYPE1>
    + std::ops::Shr<usize, Output = Self::CHUNK_TYPE1>
    + std::ops::Sub<Output = Self::CHUNK_TYPE1>
    + std::ops::SubAssign;

    const CHUNK_SIZE: usize;
    const ZERO: Self::CHUNK_TYPE1;
    const ONE: Self::CHUNK_TYPE1;
    unsafe fn buildChunk(chunk: &mut TokenizerChunkMask<Self>, ptr: *const u8)
    where
        Self: Sized;
    fn trailingZeros(mask: Self::CHUNK_TYPE1) -> u32;
}
pub trait TokenizerBackendAVX2: TokenizerBackend<CHUNK_TYPE1 = u32> {}
pub trait TokenizerBackendAVX512: TokenizerBackend<CHUNK_TYPE1 = u64> {}

impl private::TokenizerBackendPrivate for AVX2 {}
impl TokenizerBackend for AVX2 {
    type CHUNK_TYPE1 = u32;
    const CHUNK_SIZE: usize = u32::BITS as usize;

    const ZERO: u32 = 0;
    const ONE: u32 = 1u32;
    unsafe fn buildChunk(chunk: &mut TokenizerChunkMask<Self>, ptr: *const u8) {
        chunk.buildAVX2(ptr);
    }
    fn trailingZeros(mask: u32) -> u32 {
        mask.trailing_zeros()
    }
}
impl TokenizerBackendAVX2 for AVX2 {}

impl private::TokenizerBackendPrivate for AVX512 {}
impl TokenizerBackend for AVX512 {
    type CHUNK_TYPE1 = u64;
    const CHUNK_SIZE: usize = u64::BITS as usize;
    const ZERO: u64 = 0;
    const ONE: u64 = 1u64;
    unsafe fn buildChunk(chunk: &mut TokenizerChunkMask<Self>, ptr: *const u8) {
        chunk.buildAVX512(ptr);
    }
    fn trailingZeros(mask: u64) -> u32 {
        mask.trailing_zeros()
    }
}
impl TokenizerBackendAVX512 for AVX512 {}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Tokenizer<TBackend>
where
TBackend: TokenizerBackend {
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
    current_chunk_mask: TokenizerChunkMask<TBackend>,
    /// Ожидающий токен. Токен, который был найден при поиске другого.
    /// Pending token. A token that was found while searching for another.
    pending_token: Token,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> Tokenizer<TBackend>
where
TBackend: TokenizerBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(data_rc: Rc<Vec<u8>>) -> Result<Self, String> {
        // Строим первый чанк.
        // Building the first chunk.
        let mut chunk_ = TokenizerChunkMask::<TBackend>::s_create();
        unsafe { TBackend::buildChunk(&mut chunk_, data_rc.as_ptr()); }

        Ok(Self {
            data_ptr: data_rc.as_ptr(),
            data_length: data_rc.deref().len(),
            data_rc: data_rc,
            state: TokenizerState::TAG_FIND,
            current_in_data_position: 0,
            current_in_chunk_position: 0,
            current_chunk_mask: chunk_,
            pending_token: Token::s_createEmpty(),
        })
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TBackend> Tokenizer<TBackend>
where
TBackend: TokenizerBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Установка новых данных. Установка приводит к полному сбросу состояния токенайзера.
    /// Installing new data. This causes a complete reset of the tokenizer state.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn setData(&mut self, data_rc: Rc<Vec<u8>>) {
        self.data_rc = data_rc;

        self.reset();
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Сброс текущего состояния токенайзера. Токенайзер приводится в начальное состояние.
    /// Resets the current state of the tokenizer. The tokenizer is returned to its initial state.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn reset(&mut self) {
        // Строим первый чанк.
        // Building the first chunk.
        let mut chunk_ = TokenizerChunkMask::<TBackend>::s_create();
        unsafe { TBackend::buildChunk(&mut chunk_, self.data_ptr); }

        self.state = TokenizerState::TAG_FIND;
        self.current_in_data_position = 0;
        self.current_in_chunk_position = 0;
        self.current_chunk_mask = chunk_;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn nextToken1(&mut self) -> Token {
        let token_ = self.processState();

        token_

        /*let mut token_type_ = TokenType::END;
        let mut token_data_rng_: Range<usize> = (0 .. 0);

        let mut end_ = false;
        let mut tag_name_rng_ : Range<usize> = (0 .. 0);
        let mut attribute_name_rng_ : Range<usize> = (0 .. 0);
        let mut attribute_value_rng_ : Range<usize> = (0 .. 0);
        let mut attributes_vec_ = Vec::<TagAttribute>::with_capacity(10);
        let mut closed_bool_ = false;

        //3275086
        if self.current_chunk_position > 3275032 {
            println!("stop");
        }

        'chunks: loop {
            'analyze: loop {
                match self.state {
                    TokenizerState::L_CHEVRON => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        // очищаем маску л-шеврона до найденной позиции
                        self.current_masks.l_chevron_mask = self.current_masks.l_chevron_mask & !((1u32 << self.current_mask_position) - 1u32);

                        // если маска пустая, останавливаем анализ, чтобы получить новые маски
                        if self.current_masks.l_chevron_mask == 0 {
                            break 'analyze;
                        }                       

                        // ищем бит. найденный бит = найденный л-шефрон
                        // количество 0 до первой 1 = позиция 1
                        let l_chevron_tz_ : u32 = self.current_masks.l_chevron_mask.trailing_zeros();
                        
                        // сохраняем позицию
                        self.current_mask_position = l_chevron_tz_ + 1;

                        token_type_ = TokenType::TAG_OPEN;
                        token_data_rng_.start = self.current_chunk_position + l_chevron_tz_ as usize;

                        // следующее состояние, анализ допустимого символа после л-шеврона
                        self.state = TokenizerState::L_CHEVRON_NEXT;

                        continue 'analyze;
                    }

                    TokenizerState::L_CHEVRON_NEXT => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = 1u32 << self.current_mask_position;

                        // Если на текущей позиции находится спецсимвол.
                        if bit_ & self.current_masks.special_mask != 0 {
                            self.state = TokenizerState::L_TAG_NAME;

                            token_type_ = TokenType::TAG_SELF_CLOSE;
                            token_data_rng_.end = self.current_chunk_position + self.current_mask_position as usize;

                            break 'chunks;
                        }

                        // если на искомой позиции находятся символы-цифры, переключаемся на новое состояние
                        else if bit_ & self.current_masks.letters_digitals_mask != 0 {
                            self.state = TokenizerState::L_TAG_NAME;

                            token_type_ = TokenType::TAG_OPEN;
                            token_data_rng_.end = token_data_rng_.start;

                            break 'chunks;
                        }

                        // 
                        else {
                            // если на искомой позиции находятся спецсимволы,
                            if bit_ & self.current_masks.special_mask != 0 {
                                // смещаем обрабатываемую позицию
                                self.current_mask_position += 1;

                                self.state = TokenizerState::L_TAG_NAME;

                                closed_bool_ = true;

                                continue 'analyze;
                            }

                            // 
                            else {
                                self.state = TokenizerState::END;
                                continue 'analyze;
                            }
                        }                                        
                    }

                    TokenizerState::L_TAG_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut invalid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask | self.current_masks.special_mask; 

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        self.current_masks.letters_digitals_mask = self.current_masks.letters_digitals_mask & !((1u32 << self.current_mask_position) - 1u32);

                        // если маски пустые
                        if self.current_masks.letters_digitals_mask | invalid_mask_ == 0 {
                            break 'analyze;
                        }

                        let invalid_tz = invalid_mask_.trailing_zeros();
                        let letters_tz = self.current_masks.letters_digitals_mask.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz <= letters_tz {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = letters_tz + 1;
                        token_data_rng_.start = self.current_chunk_position + letters_tz as usize;

                        // следующая стадия, поиск конечной границы имени тега
                        self.state = TokenizerState::R_TAG_NAME;
                    }

                    TokenizerState::R_TAG_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.separators_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask | self.current_masks.special_mask;                        

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        // если маски пустые
                        if valid_mask_ | invalid_mask_ == 0 {
                            break 'analyze;
                        }

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        token_type_ = TokenType::TAG_NAME;
                        token_data_rng_.end = self.current_chunk_position + valid_tz_ as usize;

                        // Определяем следующее состояние токенизатора.
                        // Determine the next state of the tokenizer.

                        // Если валидный бит оказался п-шевроном
                        // If the valid bit turned out to be a r-chevron
                        if self.current_masks.r_chevron_mask & (1 << valid_tz_) != 0 {
                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = TokenizerState::L_CHEVRON;

                            break 'chunks;
                        }

                        // если валидный бит оказался разделителем
                        // if the valid bit turned out to be a separator
                        else if self.current_masks.separators_mask & (1 << valid_tz_) != 0 {
                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = TokenizerState::L_ATTRIBUTE_NAME;

                            break 'chunks;
                        }

                        break 'chunks;
                    }

                    TokenizerState::L_ATTRIBUTE_NAME => {
                        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        // If the current position being processed goes beyond the limits, stop the analysis and get new chunks
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.letters_digitals_mask | self.current_masks.special_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask;

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        // clear the masks that will be used until the found position
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        // Если маски пустые
                        // If masks are empty
                        if valid_mask_ | invalid_mask_ == 0 {
                            break 'analyze;
                        }

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();

                        // Если недопустимый бит встретился раньше, ошибка парсинга
                        // If an invalid bit occurs earlier, a parse error occurs
                        if invalid_tz < valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        // Если валидный бит оказался п-шевроном
                        // If the valid bit turned out to be a r-chevron
                        if self.current_masks.r_chevron_mask & (1 << valid_tz_) != 0 {
                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = TokenizerState::L_CHEVRON;

                            token_type_ = TAG_END;
                            token_data_rng_.start = self.current_chunk_position + valid_tz_ as usize;
                            token_data_rng_.end = token_data_rng_.start;

                            break 'chunks;
                        }

                        // Если валидный бит оказался спецсимволом.
                        // If the valid bit turned out to be a special character.
                        else if self.current_masks.special_mask & (1 << valid_tz_) != 0 &&
                            self.current_masks.r_chevron_mask & (1 << valid_tz_ + 1) != 0 {

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = TokenizerState::L_CHEVRON;

                            token_type_ = TokenType::TAG_CLOSE;
                            token_data_rng_.start = self.current_chunk_position + valid_tz_ as usize;
                            token_data_rng_.end = token_data_rng_.start + 1;

                            break 'chunks;
                        }

                        // Если валидный бит оказался символом
                        // If the valid bit turned out to be a symbol
                        else {
                            token_data_rng_.start = self.current_chunk_position + valid_tz_ as usize;

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = TokenizerState::R_ATTRIBUTE_NAME;

                            continue 'analyze;
                        }
                    }

                    TokenizerState::R_ATTRIBUTE_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        // if the current position being processed goes beyond the limits, stop the analysis and get new chunks
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.separators_mask | self.current_masks.equal_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.r_chevron_mask | self.current_masks.quote_mask | self.current_masks.special_mask;

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        // clear the masks that will be used until the found position
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        // если маски пустые
                        // if masks are empty
                        if valid_mask_ | invalid_mask_ == 0 {
                            break 'analyze;
                        }

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        // if an invalid bit occurs earlier, a parse error occurs
                        if invalid_tz < valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        let pos_founded_ = self.current_chunk_position + valid_tz_ as usize;
                        attribute_name_rng_.end = pos_founded_;

                        // !!! НЕТУ ОБРАБОТКИ РАВЕНСТВА !!!
                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = TokenizerState::L_ATTRIBUTE_VALUE;

                        // токен сформирован, прерываемся
                        // the token is generated, we are interrupting
                        //break 'chunks;
                    }

                    TokenizerState::L_ATTRIBUTE_VALUE => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        // if the current position being processed goes beyond the limits, stop the analysis and get new chunks
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.quote_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.r_chevron_mask | self.current_masks.equal_mask | self.current_masks.special_mask;

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        // clear the masks that will be used until the found position
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        // если маски пустые
                        // if masks are empty
                        if valid_mask_ | invalid_mask_ == 0 {
                            break 'analyze;
                        }

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        // if an invalid bit occurs earlier, a parse error occurs
                        if invalid_tz < valid_tz_ {
                            self.state = TokenizerState::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        let pos_founded_ = 1 + self.current_chunk_position + valid_tz_ as usize;
                        attribute_value_rng_.start = pos_founded_;

                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = TokenizerState::R_ATTRIBUTE_VALUE;

                        continue 'analyze;
                    }

                    TokenizerState::R_ATTRIBUTE_VALUE => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        // if the current position being processed goes beyond the limits, stop the analysis and get new chunks
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.quote_mask;
                        //let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.r_chevron_mask | self.current_masks.equal_mask | self.current_masks.special_mask;

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        // clear the masks that will be used until the found position
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        //invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        // если маски пустые
                        // if masks are empty
                        if valid_mask_ /*| invalid_mask_*/ == 0 {
                            break 'analyze;
                        }

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        //let invalid_tz = invalid_mask_.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        // if an invalid bit occurs earlier, a parse error occurs
                        //if invalid_tz < valid_tz_ {
                        //    self.state = State::END;
                        //    continue 'analyze;
                        //}

                        self.current_mask_position = valid_tz_ + 1;

                        let pos_founded_ = self.current_chunk_position + valid_tz_ as usize;
                        attribute_value_rng_.end = pos_founded_;
                        attributes_vec_.push(TagAttribute::new(attribute_name_rng_.clone(), attribute_value_rng_.clone()));

                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = TokenizerState::L_ATTRIBUTE_NAME;

                        // токен сформирован, прерываемся
                        // the token is generated, we are interrupting
                        //break 'chunks;
                    }

                    TokenizerState::INVALID => {
                        break 'chunks;
                    }

                    TokenizerState::END => {
                        //let begin_ = self.current_chunk_position - 64;
                        //let end_ = self.current_chunk_position + 64;
                        //println!("{}", String::from_utf8_lossy(&self.data[begin_ .. end_]));

                        tag_name_rng_.end = data.len();
                        attribute_name_rng_.end = data.len();
                        attribute_value_rng_.end = data.len();

                        end_ = true;

                        break 'chunks;
                    }

                    (_) => {}

                }; // match self.state              
            } // 'analyze: loop

            self.nextChunk(&data);

            self.current_mask_position = 0;

        }; // 'chunks: loop

        (Token::s_create(token_type_, token_data_rng_), end_)*/
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
enum TokenizerState {
    TAG_FIND,
    TAG_READING,
    TAG_NAME_FIND,
    TAG_NAME_READING,
    TAG_CLOSE_READING,
    TAG_ATTRIBUTE_NAME_FINE,
    TAG_ATTRIBUTE_NAME_READING,
    TAG_ATTRIBUTE_VALUE_FINE,
    TAG_ATTRIBUTE_VALUE_READING,
    
    END,
}

macro_rules! GET_VALID_TZ {
    //($self: ident, $token_type: ident, $token_data_rng: ident,  $chunk_loop: lifetime, $analyze_loop: lifetime) => {{
    ($self: ident, $valid_mask: expr, $analyze_loop: lifetime) => {{
        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
        if $self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
            break $analyze_loop;
        }

        // Очищаем валидную и не валидную маски до найденной позиции.
        // Clear valid and invalid masks to the found position.
        $valid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);

        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
        // If the masks are empty, stop the analysis to get a new chunk.
        if $valid_mask == TBackend::ZERO {
            break $analyze_loop;
        }

        let valid_tz_ = TBackend::trailingZeros($valid_mask);

        // Сохраняем позицию.
        // Save the position.
        $self.current_in_chunk_position = valid_tz_ as usize + 1;

        valid_tz_
    }};
}

macro_rules! GET_VALID_INVALID_TZ {
    //($self: ident, $token_type: ident, $token_data_rng: ident,  $chunk_loop: lifetime, $analyze_loop: lifetime) => {{
    ($self: ident, $valid_mask: ident, $invalid_mask: ident, $analyze_loop: lifetime) => {{
        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
        if $self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
            break $analyze_loop;
        }

        // Очищаем валидную и не валидную маски до найденной позиции.
        // Clear valid and invalid masks to the found position.
        $valid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);
        $invalid_mask &= !((TBackend::ONE << $self.current_in_chunk_position) - TBackend::ONE);

        // Если маски пустые, останавливаем анализ, чтобы получить новый чанк.
        // If the masks are empty, stop the analysis to get a new chunk.
        if $valid_mask | $invalid_mask == TBackend::ZERO {
            break $analyze_loop;
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
TBackend: TokenizerBackend
{
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn processState(&mut self) -> Token {
        let mut token_type_ = TokenType::END;
        let mut token_data_rng_: Range<usize> = 0..0;
        let mut token_data_next_rng_: Range<usize> = 0..0;

        'chunk: loop {
            'analyze: loop {
                match self.state {
                    TokenizerState::TAG_FIND => {
                        let l_chevron_tz_ = GET_VALID_TZ!(self, self.current_chunk_mask.l_chevron_mask, 'analyze);

                        token_data_rng_.start = self.current_in_data_position + l_chevron_tz_ as usize;

                        // Следующее состояние, понять, какой именно тег - простой, самозакрывающийся или с инструкциями обработки.
                        // The next state is to understand what kind of tag it is - simple, self-closing, or with processing instructions.
                        self.state = TokenizerState::TAG_READING;

                        continue 'analyze;
                    } // TokenizerState::TAG_FIND

                    TokenizerState::TAG_READING => {
                        // Если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новый чанк.
                        // If the current position being processed goes out of bounds, stop analyzing and get a new chunk.
                        if self.current_in_chunk_position >= TBackend::CHUNK_SIZE {
                            break 'analyze;
                        }

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << self.current_in_chunk_position;

                        // Если на текущей позиции находится '/'.
                        // If the current position is '/'.
                        if bit_ & self.current_chunk_mask.forward_slash != TBackend::ZERO {
                            token_type_ = TokenType::TAG_BEGIN_CLOSE;
                            token_data_rng_.end = self.current_in_data_position + self.current_in_chunk_position;

                            // сохраняем позицию
                            //self.current_in_chunk_position += 1;

                            // Следующей состояние - поиск имени тега.
                            // The next state is to search for the tag name.
                            self.state = TokenizerState::TAG_NAME_FIND;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится '?'.
                        // If the current position is '?'.
                        else if bit_ & self.current_chunk_mask.question_mark != TBackend::ZERO {
                            token_type_ = TokenType::TAG_BEGIN_INSTRUCTION;
                            token_data_rng_.end = self.current_in_data_position + self.current_in_chunk_position;

                            // сохраняем позицию
                            //self.current_in_chunk_position += 1;

                            // Следующей состояние - поиск имени тега.
                            // The next state is to search for the tag name.
                            self.state = TokenizerState::TAG_NAME_FIND;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится символы.
                        // If there are characters at the current position.
                        else if bit_ & self.current_chunk_mask.letters_digitals_mask != TBackend::ZERO {
                            token_type_ = TokenType::TAG_BEGIN;
                            token_data_rng_.end = self.current_in_data_position + self.current_in_chunk_position - 1;

                            // Следующей состояние - поиск имени тега.
                            // The next state is to search for the tag name.
                            self.state = TokenizerState::TAG_NAME_FIND;

                            break 'chunk;
                        }
                    } // TokenizerState::TAG_READING

                    TokenizerState::TAG_NAME_FIND => {
                        let mut valid_mask_ = self.current_chunk_mask.letters_digitals_mask
                            | self.current_chunk_mask.forward_slash
                            | self.current_chunk_mask.question_mark;

                        let mut invalid_mask_ = self.current_chunk_mask.l_chevron_mask
                            | self.current_chunk_mask.r_chevron_mask
                            | self.current_chunk_mask.equal_mask
                            | self.current_chunk_mask.quote_mask;

                        let valid_tz_ = GET_VALID_INVALID_TZ!(
                            self,
                            valid_mask_,
                            invalid_mask_,
                            'analyze);

                        token_data_rng_.start = self.current_in_data_position + valid_tz_ as usize;

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

                        let valid_tz_ = GET_VALID_INVALID_TZ!(
                            self,
                            valid_mask_,
                            invalid_mask_,
                            'analyze);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        token_type_ = TokenType::TAG_NAME;
                        token_data_rng_.end = self.current_in_data_position + valid_tz_ as usize - 1;

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << valid_tz_ as usize;

                        // Если на текущей позиции находится '>'.
                        // If the current position is '>'.
                        if bit_ & self.current_chunk_mask.r_chevron_mask != TBackend::ZERO {
                            // Т.к. при чтении имени тега был найден следующий токен. Записываем в ожидание.
                            // Because the next token was found while reading the tag name, we write it to the waiting state.
                            self.pending_token.r#type = TokenType::TAG_END;
                            self.pending_token.data_rng.start = self.current_in_data_position + valid_tz_ as usize;

                            self.current_in_chunk_position -= 1;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_CLOSE_READING;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится '/'.
                        // If the current position is '/'.
                        else if bit_ & self.current_chunk_mask.forward_slash != TBackend::ZERO {
                            // Т.к. при чтении имени тега был найден следующий токен. Записываем в ожидание.
                            // Because the next token was found while reading the tag name, we write it to the waiting state.
                            self.pending_token.r#type = TokenType::TAG_END_CLOSE;
                            self.pending_token.data_rng.start = self.current_in_data_position + valid_tz_ as usize;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_CLOSE_READING;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится '?'.
                        // If the current position is '?'.
                        else if bit_ & self.current_chunk_mask.question_mark != TBackend::ZERO {
                            // Т.к. при чтении имени тега был найден следующий токен. Записываем в ожидание.
                            // Because the next token was found while reading the tag name, we write it to the waiting state.
                            self.pending_token.r#type = TokenType::TAG_END_INSTRUCTION;
                            self.pending_token.data_rng.start = self.current_in_data_position + valid_tz_ as usize;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_CLOSE_READING;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится ' '.
                        // If the current position is ' '.
                        else if bit_ & self.current_chunk_mask.separators_mask != TBackend::ZERO {
                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_ATTRIBUTE_NAME_FINE;

                            break 'chunk;
                        }
                    } // TokenizerState::TAG_NAME_READING

                    TokenizerState::TAG_CLOSE_READING => {
                        let valid_tz_ = GET_VALID_TZ!(self, self.current_chunk_mask.r_chevron_mask, 'analyze);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        token_type_ = self.pending_token.r#type.clone();
                        token_data_rng_.start = self.pending_token.data_rng.start;
                        token_data_rng_.end = self.current_in_data_position + valid_tz_ as usize;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_FIND;

                        break 'chunk;
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

                        let valid_tz_ = GET_VALID_INVALID_TZ!(
                            self,
                            valid_mask_,
                            invalid_mask_,
                            'analyze);

                        token_data_rng_.start = self.current_in_data_position + valid_tz_ as usize;

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = TBackend::ONE << valid_tz_ as usize;

                        // Если на текущей позиции находится символ.
                        // If there is a character at the current position.
                        if bit_ & self.current_chunk_mask.letters_digitals_mask != TBackend::ZERO {
                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_ATTRIBUTE_NAME_READING;

                            continue 'analyze;
                        }

                        // Если на текущей позиции находится '>'.
                        // If the current position is '>'.
                        else if bit_ & self.current_chunk_mask.r_chevron_mask != TBackend::ZERO {
                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            token_type_ = TokenType::TAG_END;
                            token_data_rng_.end = self.current_in_data_position + valid_tz_ as usize;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_FIND;

                            break 'chunk;
                        }

                        // Если на текущей позиции находится '/'.
                        // If the current position is '/'.
                        else if bit_ & self.current_chunk_mask.forward_slash != TBackend::ZERO {
                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            token_type_ = TokenType::TAG_END_CLOSE;
                            self.pending_token.data_rng.start = self.current_in_data_position + valid_tz_ as usize;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_CLOSE_READING;

                            continue 'analyze;
                        }

                        // Если на текущей позиции находится '?'.
                        // If the current position is '?'.
                        else if bit_ & self.current_chunk_mask.question_mark != TBackend::ZERO {
                            // Формируем токен для отправки.
                            // We are generating a token for sending.
                            token_type_ = TokenType::TAG_END_INSTRUCTION;
                            self.pending_token.data_rng.start = self.current_in_data_position + valid_tz_ as usize;

                            // Следующее состояние.
                            // Next state.
                            self.state = TokenizerState::TAG_CLOSE_READING;

                            continue 'analyze;
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

                        let valid_tz_ = GET_VALID_INVALID_TZ!(
                            self,
                            valid_mask_,
                            invalid_mask_,
                            'analyze);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        token_type_ = TokenType::ATTRIBUTE_NAME;
                        token_data_rng_.end = self.current_in_data_position + valid_tz_ as usize - 1;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_VALUE_FINE;

                        break 'chunk;
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

                        let valid_tz_ = GET_VALID_INVALID_TZ!(
                            self,
                            valid_mask_,
                            invalid_mask_,
                            'analyze);

                        token_data_rng_.start = self.current_in_data_position + valid_tz_ as usize;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_VALUE_READING;

                        continue 'analyze;
                    }

                    TokenizerState::TAG_ATTRIBUTE_VALUE_READING => {
                        // Маска для валидных значений.
                        // Mask for valid values.
                        let mut valid_mask_ = self.current_chunk_mask.quote_mask;

                        let valid_tz_ = GET_VALID_TZ!(
                            self,
                            valid_mask_,
                            'analyze);

                        // Формируем токен для отправки.
                        // We are generating a token for sending.
                        token_type_ = TokenType::ATTRIBUTE_VALUE;
                        token_data_rng_.end = self.current_in_data_position + valid_tz_ as usize;

                        // Следующее состояние.
                        // Next state.
                        self.state = TokenizerState::TAG_ATTRIBUTE_NAME_FINE;

                        break 'chunk;
                    }

                    TokenizerState::END => {
                        token_type_ = TokenType::END;
                        
                        break 'chunk;
                    }

                    _ => {}
                }
            } // 'analyze: loop

            self.nextChunk();
        } // 'chunk: loop

        let token_ = Token::s_create(token_type_, token_data_rng_);

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
            unsafe { TBackend::buildChunk(&mut self.current_chunk_mask, data_cptr_) };
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