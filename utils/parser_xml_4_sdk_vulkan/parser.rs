// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    std::arch::x86_64::*,
    crate::ChunkMask,
    crate::PresetMask,
    crate::EventType,
    crate::Event,
};

//const data : &str = "<!data><tag a=\"value\">test</tag>";
//const offset : usize = 0;
//                                  <!data><tag a="value">test</tag>
//                                  >gat/<tset>"eulav"=a gat<>atad!<
const open_chevron_mask : u32   = 0b00000100000000000000000010000001;
const close_chevron_mask : u32  = 0b10000000001000001000000001000000;
const equal_mask : u32          = 0b00000000000000000010000000000000;
const quotes_mask : u32         = 0b00000000000100000100000000000000;
const space_mask : u32          = 0b00000000000000000000100000000000;
const special_mask : u32        = 0b00001000000000000000000000000010;
const letters_mask : u32        = 0b01110011110011111001011100111100;
/*const open_chevron_mask : u32   = 0b10000000000000000000000000000000;
const close_chevron_mask : u32  = 0b10000000001000001000000001000000;
const equal_mask : u32          = 0b00000000000000000010000000000000;
const quotes_mask : u32         = 0b00000000000100000100000000000000;
const space_mask : u32          = 0b00000000000000000000100000000000;
const special_mask : u32        = 0b00001000000000000000000000000001;
const letters_mask : u32        = 0b01110011110011111001011100111100;*/

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
    pub fn create() -> Self {
        const data_0 : &str =
            "\
            <adata><tag a=\"value\">test</tag>\
            <?da>as<tag a=\"value\">test</tag>\
            <?da>as<tag a=\"value\">asd</tag><\
            asasdasdasdasdassd></asd>asdasdasd";

        let mut data_ = Vec::<u8>::with_capacity(data_0.len());
        data_.extend_from_slice(data_0.as_bytes());

        let preset_ = PresetMask::create();
        let mut chunk_ = ChunkMask::create();

        Self::buildChunk(&mut chunk_, data_.as_ptr(), &preset_);

        return Self{
            // первичная инициализация пресетов для масок
            // initial initialization of presets for masks
            preset_mask : preset_,

            data : data_,

            // первичная инициализация состояния парсера
            // initial initialization of the parser state
            state : State::L_CHEVRON,
            current_chunk_position: 0,
            current_mask_position: 0,
            current_masks : chunk_,
        };
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn nextEvent(&mut self) -> Event {
        let mut event_type_ = EventType::END;
        let mut data_begin_ : usize = 0;
        let mut data_end_ : usize = 0;

        'chunks: loop {                    
            'analize: loop {                
                match self.state {
                    State::L_CHEVRON => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analize;
                        }

                        // очищаем маску л-шеврона до найденной позиции
                        self.current_masks.l_chevron_mask = self.current_masks.l_chevron_mask & !((1u32 << self.current_mask_position) - 1u32);

                        // если маска пустая, останавливаем анализ, чтобы получить новые маски
                        if self.current_masks.l_chevron_mask == 0 {
                            break 'analize;
                        }                       

                        // ищем бит. найденный бит = найденный л-шефрон
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
                            break 'analize;
                        }

                        // устанавливаем бит на позицию, которую нужно найти
                        let bit_ = 1u32 << self.current_mask_position;

                        // если на искомой позиции находятся символы-цифры, переключаемся на новое состояние
                        if bit_ & self.current_masks.letters_digitals_mask != 0 {
                            self.state = State::L_TAG_NAME;

                            continue 'analize;
                        }

                        // 
                        else {
                            // если на искомой позиции находятся спецсимволы,
                            if bit_ & self.current_masks.special_mask != 0 {
                                // смещаем обрабатываемую позицию
                                self.current_mask_position += 1;

                                self.state = State::L_TAG_NAME;

                                continue 'analize;
                            }

                            // 
                            else {
                                self.state = State::INVALID;
                                break 'chunks;
                            }
                        }                                        
                    }

                    State::L_TAG_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analize;
                        }

                        let mut invalid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask | self.current_masks.special_mask; 

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        self.current_masks.letters_digitals_mask = self.current_masks.letters_digitals_mask & !((1u32 << self.current_mask_position) - 1u32);

                        let invalid_tz = invalid_mask_.trailing_zeros();
                        let letters_tz = self.current_masks.letters_digitals_mask.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < letters_tz {
                            self.state = State::INVALID;
                        }

                        self.current_mask_position = letters_tz + 1;
                        data_begin_ = self.current_chunk_position + letters_tz as usize;

                        // следующая стадия, поиск конечной границы имени тега
                        self.state = State::R_TAG_NAME;
                    }

                    State::R_TAG_NAME => {
                        // если текущая обрабатываемая позиция выходит за пределы, прекращаем анализ и получаем новые чанки
                        if self.current_mask_position >= u32::BITS {
                            break 'analize;
                        }

                        let mut valid_mask_ = self.current_masks.r_chevron_mask | self.current_masks.separators_mask;
                        let mut invalid_mask_ = self.current_masks.l_chevron_mask | self.current_masks.equal_mask | self.current_masks.quote_mask | self.current_masks.special_mask;                        

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        valid_mask_ = valid_mask_ & !((1u32 << self.current_mask_position) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << self.current_mask_position) - 1u32);

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();                       
                        
                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < valid_tz_ {
                            self.state = State::INVALID;
                            break 'chunks;
                        }

                        self.current_mask_position = valid_tz_ + 1;
                        data_end_ = self.current_chunk_position + valid_tz_ as usize;
                        event_type_ = EventType::TAG_NAME;

                        if self.current_masks.r_chevron_mask & (1 << valid_tz_) != 0 {
                            self.state = State::L_CHEVRON;
                            break 'chunks;
                        }
                        else {
                            self.state = State::L_CHEVRON;
                        }

                        break 'chunks;
                    }

                    State::L_ATTRIBUTE_NAME => {
                    }

                    State::R_ATTRIBUTE_NAME => {
                    }

                    State::L_ATTRIBUTE_VALUE => {
                    }

                    State::R_ATTRIBUTE_VALUE => {
                    }

                    State::INVALID => {
                        break 'chunks;
                    }

                    State::END => {
                        event_type_ = EventType::END;
                        break 'chunks;
                    }

                }; // match self.state              
            } // 'analize: loop

            self.nextChunk();

            self.current_mask_position = 0;

            /*println!("{:32b} '<'", self.current_masks.l_chevron_mask);
            println!("{:32b} '>'", self.current_masks.r_chevron_mask);
            println!("{:32b} '='", self.current_masks.equal_mask);
            println!("{:32b} '\"'", self.current_masks.quote_mask);
            println!("{:32b} ' '", self.current_masks.separators_mask);
            println!("{:32b} 'AaZz'", self.current_masks.letters_digitals_mask);
            println!("{:32b} /?!", self.current_masks.special_mask);*/

        }; // 'chunks: loop

        return Event::create(event_type_, unsafe {std::str::from_utf8_unchecked(&self.data[data_begin_ .. data_end_]) })
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

            Self::buildChunk(&mut self.current_masks, data_ptr_, &self.preset_mask);

            return;
        }

        self.state = State::END;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn buildChunk(chunk : &mut ChunkMask, data_ptr : *const u8, presets : &PresetMask) {
        let chunk_ = unsafe {
            // регистр 256, 32 байта по 8 бит, загружаем данные
            // register 256, 32 bytes of 8 bits, load data
            std::arch::x86_64::_mm256_loadu_epi8(data_ptr as *const i8)
        };

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
    fn buildMask(data : std::arch::x86_64::__m256i, symbol : std::arch::x86_64::__m256i) -> u32 {
        let mask_ = unsafe {
            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_ = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol);

            // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
            // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
            std::arch::x86_64::_mm256_movemask_epi8(found_)
        };   

        return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn buildMask3(data : std::arch::x86_64::__m256i, symbol_1 : std::arch::x86_64::__m256i, symbol_2 : std::arch::x86_64::__m256i, symbol_3 : std::arch::x86_64::__m256i) -> u32 {
        let mask_ = unsafe {
            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_1 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_1);

            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_2 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_2);

            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_3 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_3);
            
            let found_ = std::arch::x86_64::_mm256_or_si256(found_1,
                std::arch::x86_64::_mm256_or_si256(found_2, found_3));

            // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
            // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
            std::arch::x86_64::_mm256_movemask_epi8(found_)
        };   

        return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn buildMask4(data : std::arch::x86_64::__m256i, symbol_1 : std::arch::x86_64::__m256i, symbol_2 : std::arch::x86_64::__m256i, symbol_3 : std::arch::x86_64::__m256i, symbol_4 : std::arch::x86_64::__m256i) -> u32 {
        let mask_ = unsafe {
            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_1 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_1);

            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_2 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_2);

            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_3 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_3);

            // регистр 256, 32 байта по 8 бит, сравниваем
            // register 256, 32 bytes of 8 bits, compare
            let found_4 = std::arch::x86_64::_mm256_cmpeq_epi8(data, symbol_4);
            
            let found_ = std::arch::x86_64::_mm256_or_si256(
                std::arch::x86_64::_mm256_or_si256(found_1, found_2),
                std::arch::x86_64::_mm256_or_si256(found_3, found_4));

            // регистр 256, 32 байта по 8 бит, берем старший бит из каждого байта и собираем маску
            // register 256, 32 bytes of 8 bits, take the most significant bit from each byte and assemble the mask
            std::arch::x86_64::_mm256_movemask_epi8(found_)
        };   

        return mask_ as u32;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn buildMaskRng(data : __m256i, ascii_lowercase : __m256i, a : __m256i, z : __m256i) -> u32 {
        let mask_ = unsafe {
            let or_ = _mm256_or_si256(data, ascii_lowercase);
            let cmp_1_ = _mm256_cmpgt_epi8(or_, a);
            let cmp_2_ = _mm256_cmpgt_epi8(z, or_);
            let and_ = _mm256_and_si256(cmp_1_, cmp_2_);
            _mm256_movemask_epi8(and_)
        };
        
        return mask_ as u32;
    }
}