// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::error::Error;
use std::io::Read;
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    std::arch::x86_64::*,
    crate::ChunkMask,
    crate::PresetMask,
    crate::TokenType,
    crate::Token,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
enum State {
    L_CHEVRON,
    L_CHEVRON_NEXT,
    L_TAG_NAME,
    R_TAG_NAME,
    L_ATTRIBUTE_NAME,
    R_ATTRIBUTE_NAME,
    L_ATTRIBUTE_VALUE,
    R_ATTRIBUTE_VALUE,
    INVALID,
    END,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Parser {
    /// предустановленные данные для построения масок
    /// preset data for constructing masks
    preset_mask : PresetMask,
    /// данные для парсинга
    /// data for parsing
    data : Vec<u8>,
    /// текущее состояние парсера
    /// current state of the parser
    state : State,    
    /// текущее смещение для данных
    /// current offset for data
    current_chunk_position: usize,
    /// текущее смещение для чанка с масками
    /// current offset for the chunk with masks
    current_mask_position: u32,
    /// текущий набор масок
    /// current set of masks
    current_masks : ChunkMask,    
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Parser {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create() -> Result<Self, ()> {
        // загружаем данные
        let data_ = Self::loadDataFromFile("1.4.356.xml")?;

        let preset_ = PresetMask::create();

        // строим первую маску
        let mut chunk_ = ChunkMask::create();
        unsafe { Self::buildChunk(&mut chunk_, data_.as_ptr(), &preset_); }

        return Ok(Self {
            // первичная инициализация пресетов для масок
            // initial initialization of presets for masks
            preset_mask: preset_,

            data: data_,

            // первичная инициализация состояния парсера
            // initial initialization of the parser state
            state: State::L_CHEVRON,
            current_chunk_position: 0,
            current_mask_position: 0,
            current_masks: chunk_,
            }
        );
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn nextToken(&mut self) -> Token {
        let mut event_type_ = TokenType::END;
        let mut data_begin_ : usize = 0;
        let mut data_end_ : usize = 0;

        'chunks: loop {                    
            'analyze: loop {
                match self.state {
                    State::L_CHEVRON => {
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

                        // ищем бит. найденый бит = найденный л-шефрон
                        // количество 0 до первой 1 = позиция 1
                        let l_chevron_tz_ : u32 = self.current_masks.l_chevron_mask.trailing_zeros();
                        
                        // сохраняем позицию
                        self.current_mask_position = l_chevron_tz_ + 1;
                        
                        // следующее состояние, анализ допустимого символа после л-шеврона
                        self.state = State::L_CHEVRON_NEXT;
                    }

                    State::L_CHEVRON_NEXT => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = 1u32 << self.current_mask_position;

                        // если на искомой позиции находятся символы-цифры, переключаемся на новое состояние
                        if bit_ & self.current_masks.letters_digitals_mask != 0 {
                            self.state = State::L_TAG_NAME;

                            continue 'analyze;
                        }

                        // 
                        else {
                            // если на искомой позиции находятся спецсимволы,
                            if bit_ & self.current_masks.special_mask != 0 {
                                // смещаем обрабатываемую позицию
                                self.current_mask_position += 1;

                                self.state = State::L_TAG_NAME;

                                continue 'analyze;
                            }

                            // 
                            else {
                                self.state = State::END;
                                continue 'analyze;
                            }
                        }                                        
                    }

                    State::L_TAG_NAME => {
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
                            self.state = State::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = letters_tz + 1;
                        data_begin_ = self.current_chunk_position + letters_tz as usize;

                        // следующая стадия, поиск конечной границы имени тега
                        self.state = State::R_TAG_NAME;
                    }

                    State::R_TAG_NAME => {
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
                            self.state = State::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;
                        data_end_ = self.current_chunk_position + valid_tz_ as usize;
                        event_type_ = TokenType::TAG_NAME;

                        // если валидный бит оказался п-шевроном
                        // if the valid bit turned out to be a r-chevron
                        if self.current_masks.r_chevron_mask & (1 << valid_tz_) != 0 {
                            // значит тег закрылся, ищем новый тег
                            // this means the tag is closed, we're looking for a new tag

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = State::L_CHEVRON;

                            // токен сформирован, прерываемся
                            // the token is generated, we are interrupting
                            break 'chunks;
                        }

                        // если валидный бит оказался разделителем
                        // if the valid bit turned out to be a separator
                        else {
                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = State::L_ATTRIBUTE_NAME;

                            // токен сформирован, прерываемся
                            // the token is generated, we are interrupting
                            break 'chunks;
                        }
                    }

                    State::L_ATTRIBUTE_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        // if the current position being processed goes beyond the limits, stop the analysis and get new chunks
                        if self.current_mask_position >= u32::BITS {
                            break 'analyze;
                        }

                        let mut valid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.letters_digitals_mask | self.current_masks.special_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask;

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
                            self.state = State::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        // если валидный бит оказался п-шевроном
                        // if the valid bit turned out to be a r-chevron
                        if self.current_masks.r_chevron_mask & (1 << valid_tz_) != 0 {
                            // значит тег закрылся, ищем новый тег
                            // this means the tag is closed, we're looking for a new tag

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = State::L_CHEVRON;

                            continue 'analyze;
                        }

                        else if self.current_masks.special_mask & (1 << valid_tz_) != 0 {
                            // значит тег закрылся, ищем новый тег
                            // this means the tag is closed, we're looking for a new tag

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = State::L_CHEVRON;

                            continue 'analyze;
                        }

                        // если валидный бит оказался символом
                        // if the valid bit turned out to be a symbol
                        else {
                            event_type_ = TokenType::ATTRIBUTE_NAME;
                            data_begin_ = self.current_chunk_position + valid_tz_ as usize;

                            // сохраняем состояние парсера
                            // save the parser state
                            self.state = State::R_ATTRIBUTE_NAME;

                            continue 'analyze;
                        }
                    }

                    State::R_ATTRIBUTE_NAME => {
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
                            self.state = State::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        data_end_ = self.current_chunk_position + valid_tz_ as usize;

                        // !!! НЕТУ ОБРАБОТКИ РАВЕНСТВА !!!
                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = State::L_ATTRIBUTE_VALUE;

                        // токен сформирован, прерываемся
                        // the token is generated, we are interrupting
                        break 'chunks;
                    }

                    State::L_ATTRIBUTE_VALUE => {
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
                            self.state = State::END;
                            continue 'analyze;
                        }

                        self.current_mask_position = valid_tz_ + 1;

                        event_type_ = TokenType::ATTRIBUTE_VALUE;
                        data_begin_ = self.current_chunk_position + valid_tz_ as usize;

                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = State::R_ATTRIBUTE_VALUE;

                        continue 'analyze;
                    }

                    State::R_ATTRIBUTE_VALUE => {
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

                        data_end_ = self.current_chunk_position + valid_tz_ as usize;

                        // сохраняем состояние парсера
                        // save the parser state
                        self.state = State::L_ATTRIBUTE_NAME;

                        // токен сформирован, прерываемся
                        // the token is generated, we are interrupting
                        break 'chunks;
                    }

                    State::INVALID => {
                        break 'chunks;
                    }

                    State::END => {
                        //let begin_ = self.current_chunk_position - 64;
                        //let end_ = self.current_chunk_position + 64;
                        //println!("{}", String::from_utf8_lossy(&self.data[begin_ .. end_]));

                        event_type_ = TokenType::END;
                        data_end_ = self.data.len();
                        break 'chunks;
                    }

                }; // match self.state              
            } // 'analize: loop

            self.nextChunk();

            self.current_mask_position = 0;

        }; // 'chunks: loop

        return Token::create(event_type_, unsafe {std::str::from_utf8_unchecked(&self.data[data_begin_ .. data_end_]) })
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые методы
/// protected methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Parser {
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные методы
/// private methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Parser {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn nextChunk(&mut self) {
        let pos_ = self.current_chunk_position + 32;

        if self.current_chunk_position + 64 <= self.data.len() {
            self.current_chunk_position += 32;

            let data_ptr_ = unsafe { self.data.as_ptr().add(self.current_chunk_position) };

            unsafe { Self::buildChunk(&mut self.current_masks, data_ptr_, &self.preset_mask); }

            return;
        }

        self.state = State::END;
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные функции
/// private functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Parser {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadDataFromFile(name : &str) -> Result<Vec<u8>, ()> {
        // путь до файла с спецификацией вулкана - ../../external/vulkan/cargo.toml/
        // path to the file with the volcano specification - ../../external/vulkan/cargo.toml/
        let path_ = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .join("external")
            .join("vulkan")
            .join(name);

        let data_ = std::fs::read(path_)
            .map_err(|std_error| {
                println!("{}", std_error);
                ()
            }
        )?;

        return Ok(data_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildChunk(chunk : &mut ChunkMask, data_ptr : *const u8, presets : &PresetMask) {
        let chunk_ = _mm256_loadu_epi8(data_ptr as *const i8);

        // регистр 256, 32 байта по 8 бит, загружаем данные
        // register 256, 32 bytes of 8 bits, load data
        let chunk_ = _mm256_loadu_epi8(data_ptr as *const i8);

        chunk.l_chevron_mask = Self::buildMask(chunk_, presets.chevron_l);
        chunk.r_chevron_mask = Self::buildMask(chunk_, presets.chevron_r);
        chunk.equal_mask = Self::buildMask(chunk_, presets.equal);
        chunk.quote_mask = Self::buildMask(chunk_, presets.quote);
        chunk.separators_mask = Self::buildMask4(chunk_, presets.sp, presets.tab, presets.lf, presets.cr);
        chunk.letters_digitals_mask = Self::buildMaskRng(chunk_, presets.ascii_lowercase, presets.letter_a, presets.letter_z);
        chunk.special_mask = Self::buildMask3(chunk_, presets.slash, presets.qm, presets.em);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildMask(data : __m256i, symbol : __m256i) -> u32 {
            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_ = _mm256_cmpeq_epi8(data, symbol);

            // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
            // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
            let mask_ =  _mm256_movemask_epi8(found_);

            return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildMask3(data : __m256i, symbol_1 : __m256i, symbol_2 : __m256i, symbol_3 : __m256i) -> u32 {
        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let found_1 = _mm256_cmpeq_epi8(data, symbol_1);

        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let found_2 = _mm256_cmpeq_epi8(data, symbol_2);

        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let found_3 = _mm256_cmpeq_epi8(data, symbol_3);

        let found_ = _mm256_or_si256(found_1,
             _mm256_or_si256(found_2, found_3));

        // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
        // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
        let mask_ = _mm256_movemask_epi8(found_);

        return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildMask4(data : __m256i, symbol_1 : __m256i, symbol_2 : __m256i, symbol_3 : __m256i, symbol_4 : __m256i) -> u32 {
        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let cmpeq_1 = _mm256_cmpeq_epi8(data, symbol_1);

        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let cmpeq_2 = _mm256_cmpeq_epi8(data, symbol_2);

        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let cmpeq_3 = _mm256_cmpeq_epi8(data, symbol_3);

        // регистр 256, 32 байта по 8 бит, сравниваем
        // register 256, 32 bytes of 8 bits, compare
        let found_4 = _mm256_cmpeq_epi8(data, symbol_4);

        let found_ = _mm256_or_si256(
            _mm256_or_si256(cmpeq_1, cmpeq_2),
            _mm256_or_si256(cmpeq_3, found_4));

        // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
        // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
        let mask_ =_mm256_movemask_epi8(found_);

        return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe fn buildMaskRng(data : __m256i, ascii_lowercase : __m256i, a : __m256i, z : __m256i) -> u32 {
        let or_ = _mm256_or_si256(data, ascii_lowercase);
        let cmp_1_ = _mm256_cmpgt_epi8(or_, a);
        let cmp_2_ = _mm256_cmpgt_epi8(z, or_);
        let and_ = _mm256_and_si256(cmp_1_, cmp_2_);
        let mask_ =_mm256_movemask_epi8(and_);

        return mask_ as u32;
    }
}