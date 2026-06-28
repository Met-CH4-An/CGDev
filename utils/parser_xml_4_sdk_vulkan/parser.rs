// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::EventType,
    crate::Event,
};

const data : &str = "<!data><tag a=\"value\">test</tag>";
const offset : usize = 0;
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
    state : State,
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
        return Self{
            state : State::L_CHEVRON,
        };
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn parse(&mut self) -> Event {
        let mut event_type_ = EventType::END;
        let mut data_begin_ = 0;
        let mut data_end_ = 0;

        'chunks: loop {
            let [
                mut l_chevron_mask_,
                mut r_chevron_mask_,
                mut equal_mask_,
                mut quotes_mask_,
                mut tab_space_mask_,
                mut letters_mask_,
                mut special_mask_,
            ] = self.buildMasks();
            
            let mut proceed_position_ : u32 = 0;
        
            'analize: loop {
                match self.state {
                    State::L_CHEVRON => {
                        // очищаем маску л-шеврона до найденной позиции
                        l_chevron_mask_ = l_chevron_mask_ & !((1u32 << proceed_position_) - 1u32);

                        // если маска пустая, останавливаем анализ, чтобы получить новые маски
                        if l_chevron_mask_ == 0 {
                            break 'analize;
                        }                       

                        // ищем бит. найденный бит = найденный л-шефрон
                        // количество 0 до первой 1 = позиция 1
                        let l_chevron_tz_ : u32 = l_chevron_mask_.trailing_zeros();
                        
                        // сохраняем обработанную позицию
                        proceed_position_ = l_chevron_tz_ + 1;
                        
                        // следующее состояние, анализ допустимого символа после л-шеврона
                        self.state = State::L_CHEVRON_NEXT;  

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
                        }                     
                    }

                    State::L_CHEVRON_NEXT => {
                        let valid_mask_ : u32 = letters_mask_ | special_mask_;
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
                        let mut invalid_mask_ = r_chevron_mask_ | l_chevron_mask_ | equal_mask_ | quotes_mask_ | special_mask_; 

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        invalid_mask_ = invalid_mask_ & !((1u32 << proceed_position_) - 1u32);
                        letters_mask_ = letters_mask_ & !((1u32 << proceed_position_) - 1u32);

                        let invalid_tz = invalid_mask_.trailing_zeros();
                        let letters_tz = letters_mask_.trailing_zeros();

                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < letters_tz {
                            self.state = State::INVALID;
                        }

                        proceed_position_ = letters_tz + 1;

                        // следующая стадия, поиск конечной границы имени тега
                        self.state = State::R_TAG_NAME;

                        data_begin_ = proceed_position_;

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
                        }                         
                    }

                    State::R_TAG_NAME => {
                        let mut valid_mask_ = r_chevron_mask_ | tab_space_mask_;
                        let mut invalid_mask_ = l_chevron_mask_ | equal_mask_ | quotes_mask_ | special_mask_;                        

                        // очищаем маски, которые будут использоваться, до найденной позиции
                        valid_mask_ = valid_mask_ & !((1u32 << proceed_position_) - 1u32);
                        invalid_mask_ = invalid_mask_ & !((1u32 << proceed_position_) - 1u32);                        

                        let valid_tz_ = valid_mask_.trailing_zeros();
                        let invalid_tz = invalid_mask_.trailing_zeros();                       
                        
                        // если недопустимый бит встретился раньше, ошибка парсинга
                        if invalid_tz < valid_tz_ {
                            self.state = State::INVALID;
                        }

                        proceed_position_ = valid_tz_ + 1;

                        if r_chevron_mask_ & (1 << valid_tz_) != 0 {
                            self.state = State::L_CHEVRON;
                        }
                        else {
                            self.state = State::L_ATTRIBUTE_NAME;
                        }
                        
                        data_end_ = proceed_position_;

                        // если позиция была последней, прекращаем анализ и получаем новые чанки
                        if proceed_position_ == i32::BITS {
                            break 'analize;
                        }  
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
        };

        return Event::END;
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
    fn buildMasks(&self) -> [u32; 7] {
        let mut masks_ = [0; 7];

        masks_[MaskType::L_CHEVRON] = open_chevron_mask;
        masks_[MaskType::R_CHEVRON] = close_chevron_mask;
        masks_[MaskType::EQUAL] = equal_mask;
        masks_[MaskType::QUOTES] = quotes_mask;
        masks_[MaskType::TAB_SPACE] = space_mask;
        masks_[MaskType::LETTERS] = letters_mask;
        masks_[MaskType::SPECIAL] = special_mask;

        return masks_;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn clearMasks(&mut self, position : &u32, masks : &mut [u32; 7]) {
        // убираем найденный бит
        masks[0] = masks[0] & !(1u32 << *position);
        //masks[1] = masks[1] & !(1u32 << *position);
        //masks[2] = masks[2] & !(1u32 << *position);
        //masks[3] = masks[3] & !(1u32 << *position);
        //masks[4] = masks[4] & !(1u32 << *position);
        //masks[5] = masks[5] & !(1u32 << *position);
        //masks[6] = masks[6] & !(1u32 << *position);
    }
}