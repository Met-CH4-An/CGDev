// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::MaskPreset,
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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Parser {
    data : Vec<u8>,
    state : State,    
    offset : usize,
    /// текущая маска для '<'
    /// current mask for '<'
    l_chevron_mask : u32,
    /// текущая маска для '>'
    /// current mask for '>'
    r_chevron_mask : u32,
    /// текущая маска для '='
    /// current mask for '='
    equal_mask : u32,
    /// текущая маска для '"'
    /// current mask for '"'
    quotes_mask : u32,
    /// текущая маска для ' ', '\t', '\n'
    /// current mask for ' ', '\t', '\n'
    separators_mask : u32,
    /// текущая маски для 'Aa' ... 'Zz', '0' ... '9'
    /// current masks for 'Aa' ... 'Zz', '0' ... '9'
    letters_digitals_mask : u32,
    /// текущая маска для спецсимволов '/', '?', '!'
    /// current mask for special characters '/', '?', '!'
    special_mask : u32,

    mask_preset : MaskPreset,
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
        const data_0 : &str = "<!data><tag a=\"value\">test</tag>";

        let mut data_ = Vec::<u8>::with_capacity(data_0.len());
        data_.extend_from_slice(data_0.as_bytes());

        let [
            l_chevron_mask_,
            r_chevron_mask_,
            equal_mask_,
            quotes_mask_,
            separator_mask_,
            special_mask_,
        ] = Self::nextChunk(data_.as_ptr() as *const i8);
        
        return Self{
            state : State::L_CHEVRON,
            data : data_,
            offset : 0,

            // первичная инициализация масок
            l_chevron_mask : l_chevron_mask_,
            r_chevron_mask : r_chevron_mask_,
            equal_mask : equal_mask_,
            quotes_mask : quotes_mask_,
            separators_mask : separator_mask_,
            letters_digitals_mask : 0,
            special_mask : special_mask_,

            // первичная инициализация пресетов для масок
            // initial initialization of presets for masks
            mask_preset : MaskPreset::create(),
        };
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn nextEvent(&mut self) -> Event {
        let mut event_type_ = EventType::END;
        let mut data_begin_ : usize = 0;
        let mut data_end_ : usize = 10;

        'chunks: loop {            
            let mut proceed_position_ : u32 = 0;
        
            'analize: loop {
                match self.state {
                    State::L_CHEVRON => {
                        // очищаем маску л-шеврона до найденной позиции
                        self.l_chevron_mask = self.l_chevron_mask & !((1u32 << proceed_position_) - 1u32);

                        // если маска пустая, останавливаем анализ, чтобы получить новые маски
                        if self.l_chevron_mask == 0 {
                            break 'analize;
                        }                       

                        // ищем бит. найденный бит = найденный л-шефрон
                        // количество 0 до первой 1 = позиция 1
                        let l_chevron_tz_ : u32 = self.l_chevron_mask.trailing_zeros();
                        
                        // сохраняем обработанную позицию
                        proceed_position_ = l_chevron_tz_ + 1;
                        
                        // следующее состояние, анализ допустимого символа после л-шеврона
                        self.state = State::L_CHEVRON_NEXT;  

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == u32::BITS {
                            break 'analize;
                        }                     
                    }

                    State::L_CHEVRON_NEXT => {
                        let valid_mask_ : u32 = self.letters_digitals_mask | self.special_mask;
                        //let invalid_mask_ : u32 = r_chevron_mask_ | equal_mask_ | quotes_mask_ | tab_space_mask_;

                        // если на текущей позиции не допустимый символ
                        if valid_mask_ & (1u32 << proceed_position_) == 0 {
                            self.state = State::INVALID;
                            break 'chunks;
                        }

                        proceed_position_ += 1;

                        // следующая стадия, поиск начальной границы имени тега
                        self.state = State::L_TAG_NAME;

                        event_type_ = EventType::TAG_NAME;

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
                        }                   
                    }

                    State::L_TAG_NAME => {
                        let mut invalid_mask_ = self.r_chevron_mask | self.l_chevron_mask | self.equal_mask | self.quotes_mask | self.special_mask; 

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        invalid_mask_ = invalid_mask_ & !((1u32 << proceed_position_) - 1u32);
                        self.letters_digitals_mask = self.letters_digitals_mask & !((1u32 << proceed_position_) - 1u32);

                        let invalid_tz = invalid_mask_.trailing_zeros();
                        let letters_tz = self.letters_digitals_mask.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < letters_tz {
                            self.state = State::INVALID;
                        }

                        proceed_position_ = letters_tz + 1;

                        // следующая стадия, поиск конечной границы имени тега
                        self.state = State::R_TAG_NAME;

                        data_begin_ = letters_tz as usize;

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
                        }                         
                    }

                    State::R_TAG_NAME => {
                        let mut valid_mask_ = self.r_chevron_mask | self.separators_mask;
                        let mut invalid_mask_ = self.l_chevron_mask | self.equal_mask | self.quotes_mask | self.special_mask;                        

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        valid_mask_ = valid_mask_ & !((1u32 << proceed_position_) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << proceed_position_) - 1u32);                        

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();                       
                        
                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < valid_tz_ {
                            self.state = State::INVALID;
                            break 'chunks;
                        }

                        proceed_position_ = valid_tz_ + 1;

                        if self.r_chevron_mask & (1 << valid_tz_) != 0 {
                            self.state = State::L_CHEVRON;
                        }
                        else {
                            self.state = State::L_CHEVRON;
                        }
                        
                        data_end_ = valid_tz_ as usize;

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
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

                }; // match self.state              
            } // 'analize: loop          
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
    fn nextChunk() {
        
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn buildChunk(data_ptr : *const i8, presets : &MaskPreset) -> [u32; 6] {
        // указатель на текущий участок данных для построения масок
        //let data_ptr_ = unsafe {
        //    self.data.as_ptr().add(self.offset) as *const i8
        //};

        let chunk_ = unsafe {
            // регистр 256, 32 байта по 8 бит, загружаем данные
            // register 256, 32 bytes of 8 bits, load data
            std::arch::x86_64::_mm256_loadu_epi8(data_ptr)
        };

        let l_chevron_mask_ = Self::buildMask(chunk_, presets.chevron_l);
        let r_chevron_mask_ = Self::buildMask(chunk_, presets.chevron_r);
        let equal_mask_ = Self::buildMask(chunk_, presets.equal);
        let quotes_mask_ = Self::buildMask(chunk_, presets.quote);
        let separators_mask_ = Self::buildMask4(chunk_, presets.sp, presets.tab, presets.lf, presets.cr);
        let special_mask_ = Self::buildMask3(chunk_, presets.slash, presets.qm, presets.em);

        return [
            l_chevron_mask_,
            r_chevron_mask_,
            equal_mask_,
            quotes_mask_,
            separators_mask_,
            special_mask_
        ];
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
}