// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// макрос для получения форматированного описания
/// macro for getting a formatted description
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
macro_rules! getFormattedDescription {
    ($error_type : expr, $description : expr) => {{
        let location_ = std::panic::Location::caller();
        &format!("
type: {:?}
type description: {}
error description: {}
module location: {}
row number: {}
column number: {}\n",
            $error_type, $error_type.toStr(), $description, location_.file(), location_.line(), location_.column()
        )
    }};
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub trait ErrorTypeToStr {
    fn toStr(&self) -> &str;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Error<TErrorType> {
    // шаблонный тип ошибки. pattern error type
    error_type : TErrorType,
    // описание ошибки. только для дебага. Error description. For debugging purposes only.
    #[cfg(debug_assertions)]
    description : String,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TErrorType> Error<TErrorType> 
where TErrorType: Clone + std::fmt::Debug + ErrorTypeToStr {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[track_caller]
    pub fn create(error_type : TErrorType) -> Self {
        return Self::createEx(error_type, "");
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[track_caller]
    pub fn createWithDescription(error_type : TErrorType, description : &str) -> Self {
        return Self::createEx(error_type, description);
    } 

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[track_caller]
    pub fn addError(mut self, error_type : TErrorType, description : &str) -> Self {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // создаем форматированное описание через макрос и добавляем к уже существующему
        // create a formatted description using a macro and add it to the existing one
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        #[cfg(debug_assertions)]
        let description_ = getFormattedDescription!(error_type, description);

        #[cfg(debug_assertions)]
        self.description.push_str(description_);

        return self;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn getCode(&self) -> &TErrorType {
        return &self.error_type;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn getMessage(&self) -> &str {
        #[cfg(debug_assertions)]
        return &self.description;

        #[cfg(not(debug_assertions))]
        return "";
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые методы
/// protected methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TErrorType> Error<TErrorType> 
where TErrorType: Clone + std::fmt::Debug + ErrorTypeToStr {
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные методы
/// private methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TErrorType> Error<TErrorType> 
where TErrorType: Clone + std::fmt::Debug + ErrorTypeToStr {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[track_caller]
    fn createEx(error_type : TErrorType, description : &str) -> Self {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // создаем форматированное описание через макрос
        // create a formatted description using a macro
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        #[cfg(debug_assertions)]
        let description_ = getFormattedDescription!(error_type, description);

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // новый объект
        // new object
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        return Self {
            error_type : error_type,

            #[cfg(debug_assertions)]
            description : String::from(description_),
        };
    }
}