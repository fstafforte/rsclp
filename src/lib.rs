use std::any::{Any, TypeId};
use std::env::{Args, ArgsOs};
use std::fmt::{Debug, Display, Result};
use std::io::Write;
use std::io::Result as IOResult;
use std::result::Result as StdResult;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;
use std::string::ToString;

/// Error message indexes
/// if you want to set error message in your language follow the indexes order
/// see CommandLineParser::new associated function example
pub const OPTION_ALREADY_EXISTS_ERROR_IDX: usize = 0usize;
pub const SHORT_OPTION_ALREADY_EXISTS_ERROR_IDX: usize = OPTION_ALREADY_EXISTS_ERROR_IDX + 1usize;
pub const LONG_OPTION_ALREADY_EXISTS_ERROR_IDX: usize = SHORT_OPTION_ALREADY_EXISTS_ERROR_IDX + 1usize;
pub const OPTION_IS_NOT_SET_ERROR_IDX: usize = LONG_OPTION_ALREADY_EXISTS_ERROR_IDX + 1usize;
pub const OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR_IDX: usize = OPTION_IS_NOT_SET_ERROR_IDX + 1usize;
pub const OPTION_NOT_FOUND_ERROR_IDX: usize = OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR_IDX + 1usize;
pub const OPTION_TYPE_UNDEFINED_ERROR_IDX: usize = OPTION_NOT_FOUND_ERROR_IDX + 1usize;
pub const BOOLEAN_OPTION_MISMATCH_VALUE_SET_ERROR_IDX: usize = OPTION_TYPE_UNDEFINED_ERROR_IDX + 1usize;
pub const INTEGER_OPTION_MISMATCH_VALUE_SET_ERROR_IDX: usize = BOOLEAN_OPTION_MISMATCH_VALUE_SET_ERROR_IDX + 1usize;
pub const FPOINT_OPTION_MISMATCH_VALUE_SET_ERROR_IDX: usize = INTEGER_OPTION_MISMATCH_VALUE_SET_ERROR_IDX + 1usize;
pub const MISSING_OPTION_ARGUMENT_ERROR_IDX: usize = FPOINT_OPTION_MISMATCH_VALUE_SET_ERROR_IDX + 1usize;
pub const MANDATORY_OPTION_HAS_NOT_SET_ERROR_IDX: usize = MISSING_OPTION_ARGUMENT_ERROR_IDX + 1usize;
pub const OPTION_IDENTIFIER_NOT_FOUND_ERROR_IDX: usize = MANDATORY_OPTION_HAS_NOT_SET_ERROR_IDX + 1usize;
pub const OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR_IDX: usize = OPTION_IDENTIFIER_NOT_FOUND_ERROR_IDX + 1usize;
pub const CMD_LINE_OPTION_ERROR_NUM: usize = OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR_IDX + 1usize;

// Error message in the Englih default language
const OPTION_ALREADY_EXISTS_ERROR: &str = "option already exists";
const SHORT_OPTION_ALREADY_EXISTS_ERROR: &str = "single character option already exists";
const LONG_OPTION_ALREADY_EXISTS_ERROR: &str = "long form option already exists";
const OPTION_IS_NOT_SET_ERROR: &str = "is not set";
const OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR: &str ="is not of the required type";
const OPTION_NOT_FOUND_ERROR: &str = "option not found";
const OPTION_TYPE_UNDEFINED_ERROR: &str = "Undefined command line option";
const BOOLEAN_OPTION_MISMATCH_VALUE_SET_ERROR: &str = "cannot set a boolean value to this option type";
const INTEGER_OPTION_MISMATCH_VALUE_SET_ERROR: &str = "cannot set an integer value to this option type";
const FPOINT_OPTION_MISMATCH_VALUE_SET_ERROR: &str = "cannot set a floating point value to this option type";
const MISSING_OPTION_ARGUMENT_ERROR: & str = "missing option argument"; 
const MANDATORY_OPTION_HAS_NOT_SET_ERROR: &str = "mandatory option has not been set";
const OPTION_IDENTIFIER_NOT_FOUND_ERROR: &str = "option identifier not found";
const OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR: &str =  "option argument already assigned";


// Short form not set value
const EMPTY_SHORT_FORM: char = ' ';

// Command line option type names
const COMMAND_LINE_OPTION_TYPE_UNDEFINED: &str = "UndefinedCommandLineOptionType";
const COMMAND_LINE_OPTION_TYPE_BOOLEAN: &str = "BooleanCommandLineOptionType";
const COMMAND_LINE_OPTION_TYPE_INTEGER: &str = "IntegerCommandLineOptionType";
const COMMAND_LINE_OPTION_TYPE_FPOINT: &str = "FPointCommandLineOptionType";
const COMMAND_LINE_OPTION_TYPE_STRING: &str = "StringCommandLineOptionType";

// Command line option assign tas
const OPTION_ASSIGN_TAG: &str = "=";

// Comman line option type enumeration
enum CommandLineOptionType {
    UNDEFINED(&'static str),
    BOOLEAN(&'static str),
    INTEGER(&'static str),
    FPOINT(&'static str),
    STRING(&'static str)
}

// Comman line option type enumeration implementation
impl CommandLineOptionType {
    // Associated function to create  default undefined
    // command line option
    fn new() -> Self {
        Self::UNDEFINED(COMMAND_LINE_OPTION_TYPE_UNDEFINED)
    }

    // method to return the string representation
    // of command line option type
    fn unwrap(&self) -> &'static str {
        match self {
            Self::UNDEFINED(value) => { *value }
            Self::BOOLEAN(value) => { *value }
            Self::INTEGER(value) => { *value }
            Self::FPOINT(value) => { *value }
            Self::STRING(value) => { *value }
        }
    }
}

// Display trait implementation 
impl Display for CommandLineOptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.unwrap())
    }
}

// Fram<T> trait implementation to retrive a
// command line option type from a TypeId
impl From<TypeId> for CommandLineOptionType {
    fn from(type_id: TypeId) -> Self {
        let bool_type: bool = false;
        if bool_type.type_id() == type_id {
            return Self::BOOLEAN(COMMAND_LINE_OPTION_TYPE_BOOLEAN);
        }
        let i8_type = 0i8;
        if i8_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let u8_type = 0u8;
        if u8_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let i16_type = 0i16;
        if i16_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let u16_type = 0u16;
        if u16_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let i32_type = 0i32;
        if i32_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let u32_type = 0u32;
        if u32_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let i64_type = 0i64;
        if i64_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let u64_type = 0u64;
        if u64_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let i128_type = 0i128;
        if i128_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let u128_type = 0u128;
        if u128_type.type_id() == type_id {
            return Self::INTEGER(COMMAND_LINE_OPTION_TYPE_INTEGER);
        }
        let f32_type = 0.0f32;
        if f32_type.type_id() == type_id {
            return Self::FPOINT(COMMAND_LINE_OPTION_TYPE_FPOINT);
        }
        let f64_type = 0.0f64;
        if f64_type.type_id() == type_id {
            return Self::FPOINT(COMMAND_LINE_OPTION_TYPE_FPOINT);
        }
        let str_type: &str = "";
        if str_type.type_id() == type_id {
            return Self::STRING(COMMAND_LINE_OPTION_TYPE_STRING);
        }
        let string_type: String = String::new();
        if string_type.type_id() == type_id {
            return Self::STRING(COMMAND_LINE_OPTION_TYPE_STRING);
        }
        Self::UNDEFINED(COMMAND_LINE_OPTION_TYPE_UNDEFINED)
    }
}

// PartialEq traint implementation to compare
// 2 command line option types
impl PartialEq for CommandLineOptionType {
    fn eq(&self, other: &Self) -> bool {
        self.unwrap() == other.unwrap()
    }
}

// Command line option struct
// used by CommandLineParser to store
// command line options
struct CommandLineOption {
    short_form_option: char,
    long_form_option: String,
    mandatory: bool,
    arg_text: String,
    help_text: String,
    values: Vec<String>,
    typ: CommandLineOptionType,
}

// CommandLineOption implemenetation
impl CommandLineOption {
    // Associated function to create
    // a command line option identified by
    // a single charactrer only (i.g -h)
    fn new_short_only(short_form_option: char,
                        mandatory: bool,
                        arg_text: &str,
                        help_text: &str) -> Self {
        Self {
            short_form_option,
            long_form_option: String::from(""),
            mandatory,
            arg_text: String::from(arg_text),
            help_text: help_text.to_string(),
            values: vec![],
            typ: CommandLineOptionType::new()
        }
    }

    // Associated function to create
    // a command line option identified by
    // a string only (i.g --help)
    fn new_long_only(long_form_option: &str,
                        mandatory: bool,
                        arg_text: &str,
                        help_text: &str) -> Self {
        Self {
            short_form_option:  EMPTY_SHORT_FORM,
            long_form_option:  String::from(long_form_option),
            mandatory,
            arg_text: String::from(arg_text),
            help_text: help_text.to_string(),
            values: vec![],
            typ: CommandLineOptionType::new()
        }
    }

    // Associated function to create
    // a command line option identified by
    // a single character or a string (i.g -h/--help)
    fn new(short_form_option: char,
            long_form_option: &str,
            mandatory: bool,
            arg_text: &str,
            help_text: &str) -> Self {
        Self {
            short_form_option,
            long_form_option:  String::from(long_form_option),
            mandatory,
            arg_text: String::from(arg_text),
            help_text: help_text.to_string(),
            values: vec![],
            typ: CommandLineOptionType::new()
        }
    }

    // Method to calculate a unique hash value
    // that identified the stored command line option
    fn calculate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    // Method to retrive the string representation of the 
    // command line option flags
    // i.g -h/--help or --config or -v
    fn get_flags(&self) -> String {
        let mut flags_repr =  "".to_string();
        if EMPTY_SHORT_FORM != self.short_form_option {
            flags_repr = format!("-{}", self.short_form_option);
        }
        if !self.long_form_option.is_empty() {
            if !flags_repr.is_empty()  {
                flags_repr.push('/');
            }
            flags_repr.push_str(&format!("--{}", self.long_form_option));
        }
        flags_repr
    }

    // Method to retrive command line option type
    // as a text
    fn get_type_name(&self) -> String {
        self.typ.unwrap().to_string()
    }

    // Method to retrive command line option help text
    // argument passed to this method are used
    // to allign the help text
    // * `max_flags_len` - longest flags text calculated by commnad line parser
    // * `max_arg_text_len` - longest agument text calculate by command line parser
    fn help_text(&self, max_flags_len: usize, max_arg_text_len: usize) -> String {
        let mut arg_text_len = self.arg_text.len();
        
        if 0 != arg_text_len {
            let fill_len = max_arg_text_len - arg_text_len;
            let fill = std::iter::repeat(" ").take(fill_len).collect::<String>();
            format!("{:>max_flags_len$} <{:arg_text_len$}>{} {}.\n", 
                self.get_flags(), self.arg_text, fill, self.help_text)
        } else {
            arg_text_len = max_arg_text_len + 2;
            let arg_text = std::iter::repeat(" ").take(arg_text_len).collect::<String>();
            format!("{:>max_flags_len$} {:arg_text_len$} {}.\n", 
                self.get_flags(), arg_text, self.help_text)
        }
    }

    // Method that returns if a command line option
    // has be set during the parsing phase
    fn is_set(&self) -> bool {
        0 != self.values.len()
    }

    // Method taht return the first of all set values
    // ore None if no values has been set 
    fn get_value(&self) -> Option<&String> {
        if self.values.len() > 0 {
            return self.values.get(0)
        }

        None
    }

    // Method to add a value to a command line option
    // It returns Ok(()) in case passed value respect option type
    // otherwhise it returns the command line option error index
    // * `value` - Value to be added
    fn add_value(&mut self, value: &str) -> StdResult<(), usize> {
        let type_name = self.typ.unwrap();
        if type_name != COMMAND_LINE_OPTION_TYPE_UNDEFINED {
            if type_name != COMMAND_LINE_OPTION_TYPE_STRING {
                if let Ok(_) = value.parse::<bool>() {
                    if type_name != COMMAND_LINE_OPTION_TYPE_BOOLEAN {
                        return Err(BOOLEAN_OPTION_MISMATCH_VALUE_SET_ERROR_IDX);    
                    }
                }  else if let Ok(_) = value.parse::<i128>() {
                    if type_name != COMMAND_LINE_OPTION_TYPE_INTEGER {
                        return Err(INTEGER_OPTION_MISMATCH_VALUE_SET_ERROR_IDX);
                    }
                } else if let Ok(_) = value.parse::<f64>() {
                    if type_name != COMMAND_LINE_OPTION_TYPE_FPOINT {
                        return Err(FPOINT_OPTION_MISMATCH_VALUE_SET_ERROR_IDX);
                    }
                }
            }
        } else {
            return Err(OPTION_TYPE_UNDEFINED_ERROR_IDX);
        }


        self.values.push(value.to_string());
        Ok(())
    }
    fn get_values(&self) -> &Vec<String> {
       &self.values
    }
}


// Hash trait implementation of command line option
// hash value is based on short & long form flags  
impl Hash for CommandLineOption {
    fn hash<H: Hasher>(&self, state: &mut H) {

        if EMPTY_SHORT_FORM != self.short_form_option {
            self.short_form_option.hash(state)
        }
        if !self.long_form_option.is_empty() {
            self.long_form_option.hash(state);
        }
    }
}

// PartialEq trait implementation of command line option
// used to compare 2 command line options
impl PartialEq for CommandLineOption {
    fn eq(&self, other: &Self) -> bool {
        (self.typ == other.typ) &&
        (self.short_form_option == other.short_form_option) &&
        (self.long_form_option == other.long_form_option) 
    }
}



/// Command line option error representation.
/// This struct is returned by all command line parser methods
/// in case of error you can print its representation simply
/// using {} placeholder in the rust macros
///
/// # Examples
/// ```
/// use rsclp::CommandLineParser;
/// 
/// fn main() {
///     let mut clp = CommandLineParser::new(None);
///     let config_option = clp.add_string_option('c', "config", true, "file path", "configuration file path").unwrap();
///     match clp.parse_args(std::env::args()) {
///         Ok(()) => {
///         let config_file: String = clp.get_value(config_option).unwrap();
///         println!("config_file: {}", config_file);
///         },
///         Err(parse_error) => {
///             eprintln!("{}", parse_error);
///             clp.show_help();
///         }
///     }     
/// }
/// ```
#[derive(PartialEq)]
pub struct CommandLineParserError {
    flags: String,
    typ: String,
    error: String
}

impl Display for CommandLineParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{} {}: {}", self.typ, self.flags, self.error)
    }
}

impl Debug for CommandLineParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{} {}: {}", self.typ, self.flags, self.error)
    }
}


/// Command line parser parsing mode
/// DefaultParsingMode mode means that - - option is treated
/// like all other option
/// PositionalArgumentsMode menas that all the options
/// after - - are positional argument that user can pass
/// to a process launched by your application
#[derive(PartialEq)]
pub enum ParsingMode {
    DefaultParsingMode,
    PositionalArgumentsMode,
}

impl ParsingMode {

    /// This method returns a textual representation
    /// of parsing mode
    fn unwrap(&self) -> &str {
        match self {
            ParsingMode::DefaultParsingMode => {
                "DefaultParsingMode"
            },
            ParsingMode::PositionalArgumentsMode => {
                "PositionalArgumentsMode"
            }
        }
    }
}

/// Display trait of ParsingMode
impl Display for ParsingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.unwrap())
    }
}

/// Command line parser is able to parse process arguments.
/// Arguments could be of two types, single character argument (i.g -c)
/// or long text argument (i.g. --config-file)
/// Process argument could be of the following type:
/// Boolean - classic is -h to show process help, 
/// Integer - an integer value for example verbosity level --verbose 5, 
/// Floating point - a floating point numer --ratio=123.25, 
/// String - a text argument, classic configuration file path --config-file app.properties.
/// Integer, Floating point and String option has a mandatory angument
/// while Boolean option does not require an argument.
/// Argument can be passed in two ways: --config-file=config/app.properties for example
/// or --config-file config/app.properties the same example is valid for single character options
/// for example -c=onfig/app.properties or -c onfig/app.properties.
/// Single character option can be pass grouped together (i.g -xvz).
/// Be carful if a single character argument needs an argument you have to pass it or 
/// adding = and the value (i.g. -xvzf=file_to_compress.tar.gz) or as next process argument
/// (i.g -xvzf file_to_compress.tar.gz).
/// No more than a single character option with a mandatory argument can be grouped.
pub struct CommandLineParser {
    program_name: String,
    options: Vec<CommandLineOption>,
    errors_list: Vec<String>,
    remaining_args: Vec<String>,
    positional_args: Vec<String>,
    parsing_mode: ParsingMode
}


impl CommandLineParser {
    /// Associated funtion to create a CommandLineParser
    /// * `errors_list` - Optional list of errors (None means default Elnglish language)
    /// # Examples
    /// ```
    /// use rsclp::{CommandLineParser, CommandLineParserError, CMD_LINE_OPTION_ERROR_NUM};
    /// 
    /// fn main()  {
    ///     let it_error_list: [&str; CMD_LINE_OPTION_ERROR_NUM] = [
    ///        "l'opzione già esiste",
    ///         "l'opzione a singolo charattere già esiste",
    ///         "l'opzione a formato lungo già esiste",
    ///         "non è valorizzata",
    ///         "non è del tipo richiesto",
    ///         "opzione non trovata",
    ///         "opzione di linea di comando non definita",
    ///         "valore booleano non valido per questo tipo di opzione",
    ///         "valore intero non valido per questo tipo di opzione",
    ///         "valore a virgola mobile non valido per questo tipo di opzione",
    ///         "manca l'argomento per questa opzione", 
    ///         "opzione obbligatoria non valorizzata",
    ///         "identificativo dell'opzione non trovato",
    ///         "argomento dell'opzione fia assegnato"
    ///     ];
    ///     let mut clp = CommandLineParser::new(Some(it_error_list));
    ///     
    ///     
    /// }
    /// ```
    /// 
    pub fn new(errors_list: Option<[&str; CMD_LINE_OPTION_ERROR_NUM]>) -> Self {
        let mut result = Self {
            program_name: String::new(),
            options: vec![],
            errors_list: vec![],
            remaining_args: vec![],
            positional_args: vec![],
            parsing_mode: ParsingMode::DefaultParsingMode
        };
        match errors_list {
            Some(list) => {
                for element in list {
                    result.errors_list.push(element.to_string());
                }
            },
            None => {
                result.errors_list.push(OPTION_ALREADY_EXISTS_ERROR.to_string());
                result.errors_list.push(SHORT_OPTION_ALREADY_EXISTS_ERROR.to_string());
                result.errors_list.push(LONG_OPTION_ALREADY_EXISTS_ERROR.to_string());
                result.errors_list.push(OPTION_IS_NOT_SET_ERROR.to_string());
                result.errors_list.push(OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR.to_string());
                result.errors_list.push(OPTION_NOT_FOUND_ERROR.to_string());
                result.errors_list.push(OPTION_TYPE_UNDEFINED_ERROR.to_string());
                result.errors_list.push(BOOLEAN_OPTION_MISMATCH_VALUE_SET_ERROR.to_string());
                result.errors_list.push(INTEGER_OPTION_MISMATCH_VALUE_SET_ERROR.to_string());
                result.errors_list.push(FPOINT_OPTION_MISMATCH_VALUE_SET_ERROR.to_string());
                result.errors_list.push(MISSING_OPTION_ARGUMENT_ERROR.to_string());
                result.errors_list.push(MANDATORY_OPTION_HAS_NOT_SET_ERROR.to_string());
                result.errors_list.push(OPTION_IDENTIFIER_NOT_FOUND_ERROR.to_string());
                result.errors_list.push(OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR.to_string());
            }
        }
        result
    }

    /// Method to add a boolean command line option identified
    /// by a single character only
    /// * `short_form_option` - single character option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `help_text` - command line option halt text 
    pub fn add_short_boolean_option(&mut self,
        short_form_option: char,
        mandatory: bool,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {

        let mut option = CommandLineOption::new_short_only(short_form_option, 
                                                                                mandatory, 
                                                                                "", 
                                                                                help_text);
        let bool_type = false;
        option.typ = CommandLineOptionType::from(bool_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a boolean command line option identified
    /// by a long text only
    /// * `long_form_option` - long text option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `help_text` - command line option halt text 
    pub fn add_long_boolean_option(&mut self,
        long_form_option: &str,
        mandatory: bool,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new_long_only(long_form_option, 
                                                                            mandatory, 
                                                                            "", 
                                                                            help_text);
        let bool_type = false;
        option.typ = CommandLineOptionType::from(bool_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a boolean command line option identified
    /// by a single character and long form text
    /// * `short_form_option` - single character option flag
    /// * `long_form_option` - long text option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `help_text` - command line option halt text 
    pub fn add_boolean_option(&mut self,
        short_form_option: char,
        long_form_option: &str,
        mandatory: bool,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new(short_form_option,
                                                                    long_form_option, 
                                                                    mandatory, 
                                                                    "", 
                                                                    help_text);
        let bool_type = false;
        option.typ = CommandLineOptionType::from(bool_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add an integer command line option identified
    /// by a single character only
    /// * `short_form_option` - single character option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_short_integer_option(&mut self,
        short_form_option: char,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {

        let mut option = CommandLineOption::new_short_only(short_form_option, 
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let integer_type = 0i128;
        option.typ = CommandLineOptionType::from(integer_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }
   
    /// Method to add an integer command line option identified
    /// by a long text only
    /// * `long_form_option` - long text option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_long_integer_option(&mut self,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new_long_only(long_form_option, 
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let integer_type = 0i128;
        option.typ = CommandLineOptionType::from(integer_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add an integer command line option identified
    /// by a single character and long form text
    /// * `short_form_option` - single character option flag
    /// * `long_form_option` - long text option flag
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_integer_option(&mut self,
        short_form_option: char,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new(short_form_option,
                                                                    long_form_option, 
                                                                    mandatory, 
                                                                    arg_text, 
                                                                    help_text);
        let integer_type = 0i128;
        option.typ = CommandLineOptionType::from(integer_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a floating point command line option identified
    /// by a single character only
    /// * `short_form_option` - single character option flag
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_short_fpoint_option(&mut self,
        short_form_option: char,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {

        let mut option = CommandLineOption::new_short_only(short_form_option,
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let fpoint_type = 0.0f64;
        option.typ = CommandLineOptionType::from(fpoint_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a floating point command line option identified
    /// by a long text only
    /// * `long_form_option` - long text option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_long_fpoint_option(&mut self,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new_long_only(long_form_option, 
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let fpoint_type = 0.0f64;
        option.typ = CommandLineOptionType::from(fpoint_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a floating point command line option identified
    /// by a single character and long form text
    /// * `short_form_option` - single character option flag
    /// * `long_form_option` - long text option flag
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_fpoint_option(&mut self,
        short_form_option: char,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new(short_form_option,
                                                                    long_form_option, 
                                                                    mandatory, 
                                                                    arg_text, 
                                                                    help_text);
        let fpoint_type = 0.0f64;
        option.typ = CommandLineOptionType::from(fpoint_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a text command line option identified
    /// by a single character only
    /// * `short_form_option` - single character option flag
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_short_string_option(&mut self,
        short_form_option: char,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {

        let mut option = CommandLineOption::new_short_only(short_form_option,
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let str_type = "";
        option.typ = CommandLineOptionType::from(str_type.type_id());

        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a text command line option identified
    /// by a long text only
    /// * `long_form_option` - long text option flag
    /// * `mandatory` - boolean value to set if command line option is mandatory or not
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_long_string_option(&mut self,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new_long_only(long_form_option, 
                                                                                mandatory, 
                                                                                arg_text, 
                                                                                help_text);
        let str_type = "";
        option.typ = CommandLineOptionType::from(str_type.type_id());
    
        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }

    /// Method to add a text command line option identified
    /// by a single character and long form text
    /// * `short_form_option` - single character option flag
    /// * `long_form_option` - long text option flag
    /// * `arg_text` - command line option argument text to explain argument in help text 
    /// * `help_text` - command line option halt text 
    pub fn add_string_option(&mut self,
        short_form_option: char,
        long_form_option: &str,
        mandatory: bool,
        arg_text: &str,
        help_text: &str) -> StdResult<u64, CommandLineParserError> {
        let mut option = CommandLineOption::new(short_form_option,
                                                                    long_form_option, 
                                                                    mandatory, 
                                                                    arg_text, 
                                                                    help_text);
        let str_type = "";
        option.typ = CommandLineOptionType::from(str_type.type_id());
    
        if let StdResult::Err(error) = self.check_option_already_exists(&option) {
            return StdResult::Err(error);
        }

        let option_hash: u64 = option.calculate_hash();
        self.options.push(option);

        Ok(option_hash)
    }


    /// Method to add the classic help command line option
    /// -h & --help
    /// * `help_text` - command line option halt text 
    pub fn add_help_option(&mut self, help_text: &str) -> StdResult<u64, CommandLineParserError> {
        self.add_boolean_option('h', 
                                "help", 
                                false, help_text)
    }

    /// Method to add the classic version command line option
    /// -v & --version
    /// * `help_text` - command line option halt text 
    pub fn add_version_option(&mut self, help_text: &str) -> StdResult<u64, CommandLineParserError> {
        self.add_boolean_option('v', 
                                "version", 
                                false, help_text)
    }

    /// Method to check if the parser found and set a command line option
    /// during command line parsing phase
    /// * `option_hash` - command line option identifier returned by
    /// an add_* method 
    pub fn is_set(&self, option_hash: &u64) ->  bool {
        let mut iter = self.options.iter();
        while let Some(option) = iter.next() {
            if *option_hash == option.calculate_hash() {
                return option.is_set();
            }
        }
        false
    }

    /// Generic method to get the value of a command line option
    /// (the first encountered) during command line parsing phase
    /// if command line option is not of the required type &ltT&gt or
    /// is not set error is returned
    /// * `option_hash` - command line option identifier returned by
    /// an add_* method 
    pub fn get_value<T: FromStr + 'static>(&self, option_hash: u64) -> StdResult<T, CommandLineParserError> {
        let mut iter = self.options.iter();
        while let Some(option) = iter.next() {
            if option_hash == option.calculate_hash() {
                let option_value = option.get_value();
                if let Some(value) = option_value {
                    if let Ok(parsed_value) = value.parse::<T>() {
                        let type_id = parsed_value.type_id();
                        let parsed_value_type = CommandLineOptionType::from(type_id);
                        if parsed_value_type == option.typ {
                            return Ok(parsed_value);
                        }
                    }
                    return Err(CommandLineParserError {
                        flags: option.get_flags(),
                        typ: option.get_type_name(),
                        error: self.errors_list[OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR_IDX].clone()
                    });
                }
                return Err(CommandLineParserError {
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: self.errors_list[OPTION_IS_NOT_SET_ERROR_IDX].clone()
                });
            }
        }

        Err(CommandLineParserError {
            flags: String::new(),
            typ: COMMAND_LINE_OPTION_TYPE_UNDEFINED.to_string(),
            error: self.errors_list[OPTION_IDENTIFIER_NOT_FOUND_ERROR_IDX].clone()
        })
    } 

    /// Generic method to get all values of a command line option
    /// during command line parsing phase
    /// i.g if the option is -f/--input-file &ltfile name&gt input file to be merged 
    /// and the command line is -f file1.txt -o output.txt --input-file=file2.txt
    /// get_value returns a vector containing file1.txt & file2.txt 
    /// if command line option is not of the required type &ltT&gt or
    /// is not set error is returned
    /// * `option_hash` - command line option identifier returned by
    /// an add_* method 
    pub fn get_values<T: FromStr>(&self, option_hash: u64) -> Option<Vec<T>> {
        let mut values: Vec<T> = vec![];
        for option in &self.options {
            if option_hash == option.calculate_hash() {
                let option_values = option.get_values();
                for option_value in option_values {
                    if let Ok(parsed_value) = option_value.parse::<T>() {
                        values.push(parsed_value);
                    } else {
                        eprintln!("{} {} {}", option.get_type_name(), option.get_flags(), self.errors_list[OPTION_IS_NOT_OF_REQUIRED_TYPE_ERROR_IDX]); 
                        return None;
                    }
                }
            }
        }
        if values.len() > 0 {
            Some(values)
        } else {
            None
        }
    }

    /// Method to retrive the global command line help text
    pub fn get_help_text(&self) -> String {
        let mut result = String::new();
        if !self.program_name.is_empty() {
            result.push_str(&format!("{} [OPTIONS]:\n", self.program_name));
        }
        for option in &self.options {
            result.push_str(&format!("\t{}", option.help_text(self.get_max_flags_len(), 
                                                                    self.get_max_arg_text_len())));
        }

        result
    }

    /// Method to show on standard output 
    /// the global command line help text
    pub fn show_help(&self) {
        println!("{}", self.get_help_text());
    }

    /// Method to show on object that implements
    /// the std::io::Write trait 
    /// the global command line help text
    /// * `writer` - Write trait to show help text
    /// Writer::write_all is used
    pub fn show_help_on(&self, writer: &mut dyn Write) -> IOResult<()>{
        writer.write_all(self.get_help_text().as_bytes())
    }

    /// Method to retrieve all arguments not related to an option
    pub fn get_remaining_args(&self) -> &Vec<String> {
        &self.remaining_args
    }

    /// Method to retrieve all positionale arguments 
    /// if ParsingMode is DefaultParsingMode the returned
    /// vector is empty
    pub fn get_positional_args(&self) ->&Vec<String> {
        &self.positional_args
    }

    /// Method to set parsing mode
    /// * `parsing_mode` - command line parser parsing mode 
    pub fn set_parsing_mode(&mut self, parsing_mode: ParsingMode) {
        self.parsing_mode = parsing_mode;
    }

    /// Method to parse arguments of a process, 
    /// * `args_os` - an iterator over the arguments of a process, yielding an OsString value for each argument.
    pub fn parse_args_os(&mut self, args_os: ArgsOs) -> StdResult<(), CommandLineParserError> {
        let process_args: Vec<String> = args_os.map(|elem| { 
            elem.into_string().unwrap_or(String::new()) }).collect();
            self.parse(&process_args)
    }

    /// Method to parse arguments of a process, 
    /// * `args` - an iterator over the arguments of a process, yielding a String value for each argument.
    pub fn parse_args(&mut self, args: Args) -> StdResult<(), CommandLineParserError> {
        let process_args: Vec<String> = args.collect();
        self.parse(&process_args)
    }

    /// Method to process arguments of a process.
    /// As opposed to parse_args method, process controls the returned 
    /// value of the parse_args method and in case of error it shows
    /// the error and the help text on standar error 
    pub fn process(&mut self) {
        if let Err(parse_error) = self.parse_args(std::env::args()) {
            eprintln!("{}", parse_error);            
            let _ = self.show_help_on(&mut std::io::stderr().lock());
            std::process::exit(-1);
        }
    }

    /// Method to process arguments of a process.
    /// Performs the same functionality as the process method but using ArgOs
    pub fn process_os(&mut self) {
        if let Err(parse_error) = self.parse_args_os(std::env::args_os()) {
            eprintln!("{}", parse_error);            
            let _ = self.show_help_on(&mut std::io::stderr().lock());
            std::process::exit(-1);
        }
    }

    // Method that realizes the process arguments parsing
    // * `process_args` - process arguments as a strings vector reference
    fn parse(&mut self, process_args: &Vec<String>) -> StdResult<(), CommandLineParserError> {
        let mut args = process_args.clone();
        self.program_name = args[0].clone();
        args.remove(0);
        let mut dashdash: bool = false;
        let mut idx: usize = 0;
        let args_len: usize = args.len();
        while idx < args_len {
            let arg: &String = &args[idx];
            if !dashdash {
                if arg.starts_with("-") {
                    if arg.starts_with("--") {
                        self.parse_long_form_option(&args, arg, &mut idx, &mut dashdash)?;
                    } else {
                        self.parse_short_form_option(&args, arg, &mut idx)?;
                    }
                } else {
                    self.remaining_args.push(arg.clone());
                }
            } else {
                self.positional_args.push(arg.clone());
            }

            idx = idx + 1;
        }

        self.check_mandatory_options()?;

        Ok(())
    }

    // Method to check if mandatory options
    // has at least a value set
    fn check_mandatory_options(&self) -> StdResult<(), CommandLineParserError> {
        let mut iter = self.options.iter();
        while let Some(option) = iter.next() {
            if option.mandatory &&  !option.is_set() {
                return Err(CommandLineParserError {
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: self.errors_list[MANDATORY_OPTION_HAS_NOT_SET_ERROR_IDX].clone()
                });
            }
        }
        Ok(())
    }


    // Method to parse single character option
    // has at least a value set
    // * `args` - remaining process arguments as a strings vector reference
    // * `arg` - current process argument to be parsed
    // * `idx` - mutable  process argument index
    fn parse_short_form_option(&mut self, args: &Vec<String>, arg: &String, idx: &mut usize) -> StdResult<(), CommandLineParserError> {
        let args_len = args.len();
        let mut opt_arg = String::new();
        let pos = arg.find(OPTION_ASSIGN_TAG).unwrap_or_else(|| {usize::MAX});
        let mut opt = &arg[1..];
        if pos != usize::MAX {
            opt = &arg[1..pos];
            opt_arg.push_str(&arg[pos + 1..]);
        }

        let options: Vec<char> = opt.chars().collect();
  
        let mut opt_arg_assigned = false;
        let mut char_idx = 0usize;
        while char_idx < options.len() {

            let opt = format!("-{}", options[char_idx]);
            if let Some(option) = self.get_option_mut(&opt) {
                if option.typ.unwrap() == COMMAND_LINE_OPTION_TYPE_BOOLEAN {
                    let mut value = "true";
                    println!("pos: {}, char_idx: {}", pos, char_idx);
                    if !opt_arg.is_empty() {
                        if !opt_arg_assigned {
                            value = &opt_arg;
                            opt_arg_assigned = true;
                        } else {
                            return Err(CommandLineParserError {
                                flags: option.get_flags(),
                                typ: option.get_type_name(),
                                error: format!("{}", self.errors_list[OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR_IDX])
                            });    
                        }
                    }
                    if let Err(error_idx) = option.add_value(value) {
                        return Err(CommandLineParserError {
                            flags: option.get_flags(),
                            typ: option.get_type_name(),
                            error: format!("{}", self.errors_list[error_idx])
                        });
                    }
                } else {
                    if opt_arg.is_empty() {
                        if *idx < args_len - 1 {
                            *idx = *idx + 1;
                            opt_arg = args[*idx].clone();
                        }
                    }
                    if opt_arg.is_empty() {
                        return Err(CommandLineParserError {
                            flags: option.get_flags(),
                            typ: option.get_type_name(),
                            error: format!("{}", self.errors_list[MISSING_OPTION_ARGUMENT_ERROR_IDX])
                        });
                    }
                    if !opt_arg_assigned {
                        if let Err(error_idx) = option.add_value(&opt_arg) {
                            return Err(CommandLineParserError {
                                flags: option.get_flags(),
                                typ: option.get_type_name(),
                                error: format!("{}", self.errors_list[error_idx])
                            });
                        }
                        opt_arg_assigned = true;
                    } else {
                        return Err(CommandLineParserError {
                            flags: option.get_flags(),
                            typ: option.get_type_name(),
                            error: format!("{}", self.errors_list[OPTION_ARGUMENT_ALREADY_ASSIGNED_ERROR_IDX])
                        });
                    }
                }
            }    
            char_idx = char_idx + 1usize;
        }
        Ok(())
    }

    // Method to parse long text option
    // has at least a value set
    // * `args` - remaining process arguments as a strings vector reference
    // * `arg` - current process argument to be parsed
    // * `idx` - mutable  process argument index
    fn parse_long_form_option(&mut self, args: &Vec<String>, arg: &String, idx: &mut usize, dashdash: &mut bool) -> StdResult<(), CommandLineParserError> {
        let args_len = args.len();
        if 2 == arg.len() {
            if self.parsing_mode == ParsingMode::PositionalArgumentsMode {
                *dashdash = true;
            }
            return Ok(());
        }
        
        let mut opt_and_arg: Vec<String> = arg.splitn(2, OPTION_ASSIGN_TAG)
                                            .map(|x| x.to_string()).collect();
        let opt = opt_and_arg.get(0).unwrap();
        if let Some(option) = self.get_option_mut(opt) {
            let mut opt_arg;
            if opt_and_arg.len() < 2 {
                if option.get_type_name() == COMMAND_LINE_OPTION_TYPE_BOOLEAN {
                   opt_arg = "true";
                } else {
                    *idx = *idx + 1;
                    if *idx < args_len {
                       opt_arg = args[*idx].as_str();     
                    } else {
                        return Err(CommandLineParserError{
                            flags: option.get_flags(),
                            typ: option.get_type_name(),
                            error: format!("{}", self.errors_list[MISSING_OPTION_ARGUMENT_ERROR_IDX])
                        });                    
                    }
                }
                opt_and_arg.push(opt_arg.to_string());
            }
            opt_arg = opt_and_arg[1].as_str();
            if opt_arg.is_empty() {
                return Err(CommandLineParserError{
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: format!("{}", self.errors_list[MISSING_OPTION_ARGUMENT_ERROR_IDX])
                });                    
            }
            if let Err(error_idx) = option.add_value(opt_arg) {
                return Err(CommandLineParserError {
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: format!("{}", self.errors_list[error_idx])
                });
            }
        } else {
            return Err(CommandLineParserError{
                flags: arg.clone(),
                typ: COMMAND_LINE_OPTION_TYPE_UNDEFINED.to_string(),
                error: format!("{}", self.errors_list[OPTION_NOT_FOUND_ERROR_IDX])
            });                    
        }
        Ok(())
    }

    // Method to get a mutable reference to an existing option
    // It returns an option so in case of a not added option
    // the None value is returned
    // * `searched_option` - option flag text to be searched 
    fn get_option_mut(&mut self, searched_option: &String) -> Option<&mut CommandLineOption> {
        let is_long = searched_option.starts_with("--");
        let mut iter = self.options.iter_mut();
        while let Some(option) = iter.next() {
            if is_long {
                if !option.long_form_option.is_empty() {
                    if option.long_form_option == searched_option[2..] {
                        return Some(option)
                    }
                }
            } else {
                if EMPTY_SHORT_FORM != option.short_form_option {
                    if option.short_form_option == searched_option.chars().nth(1).unwrap_or_else(|| {' '}) {
                        return Some(option);
                    }
                }
            }
        }
        None
    }



    // Method to check if a command line option already exists
    // or there are other option with the same single character or long text
    // flags. It returns an Ok(()) in case option not exists already 
    // a CommandLineParserErr otherwise
    // * `option_to_check` - reference to an option to be checkd
    fn check_option_already_exists(&self, option_to_check: &CommandLineOption) -> StdResult<(), CommandLineParserError> {
        let mut iter  = self.options.iter();
        while let Some(option) = iter.next() {

            if option_to_check == option {
                return Err(CommandLineParserError{
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: self.errors_list[OPTION_ALREADY_EXISTS_ERROR_IDX].to_string()
                });
            }

            if option_to_check.short_form_option == option.short_form_option {
                    return StdResult::Err(CommandLineParserError {
                        flags: option.get_flags(),
                        typ: option.get_type_name(),
                        error: self.errors_list[SHORT_OPTION_ALREADY_EXISTS_ERROR_IDX].to_string()
                    });
            }
            if option_to_check.long_form_option == option.long_form_option {
                return StdResult::Err(CommandLineParserError {
                    flags: option.get_flags(),
                    typ: option.get_type_name(),
                    error: self.errors_list[LONG_OPTION_ALREADY_EXISTS_ERROR_IDX].to_string()
                });
            }
        }
        Ok(())
    }

    // Method that returns the maximum length
    // of all options flags, used to allign the help text
    pub fn get_max_flags_len(&self) -> usize {
        let mut result = 0usize;
        for option in &self.options {
            let flags = option.get_flags();
            if result < flags.len() {
                result = flags.len();
            }
        }
        result
    }

    // Method that returns the maximum length
    // of all argument texts, used to allign the  help text
    fn get_max_arg_text_len(&self) -> usize {
        let mut result = 0usize;
        for option in &self.options {
            if result < option.arg_text.len() {
                result = option.arg_text.len();
            }
        }
        result
    }



}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn short_form_option_already_exists() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let result = clp.add_short_boolean_option('v', false, "print-out application version");
        assert_eq!(String::from("BooleanCommandLineOptionType -v/--version: single character option already exists"), format!("{}", result.unwrap_err()));
    }

    #[test]
    fn long_form_option_already_exists() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let result = clp.add_long_boolean_option("help", false, "print-out help menu");
        assert_eq!(String::from("BooleanCommandLineOptionType -h/--help: long form option already exists"), format!("{}", result.unwrap_err()));
    }

    #[test]
    fn both_form_option_already_exists() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let mut result = clp.add_boolean_option('h', "help", false,  "show this help");
        assert_eq!(format!("{}", result.unwrap_err()), String::from("BooleanCommandLineOptionType -h/--help: option already exists"));
        result = clp.add_boolean_option('h', "help-all", false, "show this help");
        assert_eq!(format!("{}", result.unwrap_err()), String::from("BooleanCommandLineOptionType -h/--help: single character option already exists"));
        result = clp.add_boolean_option('H', "help", false, "show this help");
        assert_eq!(format!("{}", result.unwrap_err()), String::from("BooleanCommandLineOptionType -h/--help: long form option already exists"));
    }

    #[test]
    fn option_help_text() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_integer_option('V', "verbose", false, "log verbosity", "set verbosity text").unwrap();
        let help_text = "\
\t-v/--version                 print-out application version.
\t   -h/--help                 print-out help menu.
\t-V/--verbose <log verbosity> set verbosity text.\n";
        assert_eq!(clp.get_help_text(), help_text);
    }

    #[test]
    fn show_help() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let args = vec!["test_show_help".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        clp.show_help();
        let _ = clp.show_help_on(&mut std::io::stderr().lock());
    }

    #[test]
    fn is_not_set() {
        let mut clp = CommandLineParser::new(None);
        let config_option = clp.add_string_option('c', "config", false, "file path", "application configuration file").unwrap();
        let args = vec!["is_not_set".to_string()];
        assert_eq!(Ok(()), clp.parse(&args));
        assert_eq!(false, clp.is_set(&config_option));
    }

    #[test]
    fn missing_madatory_option() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", true, "file path", "application configuration file").unwrap();
        let args = vec!["is_not_set".to_string()];
        assert_eq!("StringCommandLineOptionType -c/--config: mandatory option has not been set".to_string(), 
                    clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn missing_option_argument1() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", true, "file path", "application configuration file").unwrap();
        let args = vec!["missing_option_argument".to_string(),
            "-c".to_string()
        ];
        assert_eq!("StringCommandLineOptionType -c/--config: missing option argument".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn missing_option_argument2() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", true, "file path", "application configuration file").unwrap();
        let args = vec!["missing_option_argument".to_string(),
            "-c=".to_string()
        ];
        assert_eq!("StringCommandLineOptionType -c/--config: missing option argument".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn missing_option_argument3() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", true, "file path", "application configuration file").unwrap();
        let args = vec!["missing_option_argument".to_string(),
            "--config".to_string()
        ];
        assert_eq!("StringCommandLineOptionType -c/--config: missing option argument".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn missing_option_argument4() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", true, "file path", "application configuration file").unwrap();
        let args = vec!["missing_option_argument".to_string(),
            "--config=".to_string()
        ];
        assert_eq!("StringCommandLineOptionType -c/--config: missing option argument".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn option_is_not_set() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let verbosity = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let args = vec!["test_sho_help".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        let value = clp.get_value::<i32>(verbosity);
        assert_eq!("IntegerCommandLineOptionType -V/--verbose: is not set".to_string(), value.unwrap_err().to_string());
    }

    #[test]
    fn option_identifier_is_undefined() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let args = vec!["test_sho_help".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        let value = clp.get_value::<i32>(1u64);
        assert_eq!("UndefinedCommandLineOptionType : option identifier not found".to_string(), value.unwrap_err().to_string());
    }

    #[test]
    fn option_get_mismatch_value() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let verbosity = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let args = vec!["test_sho_help".to_string(), "-V".to_string(), "-12".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        let value = clp.get_value::<f32>(verbosity);
        assert_eq!("IntegerCommandLineOptionType -V/--verbose: is not of the required type".to_string(), value.unwrap_err().to_string());
        let value = clp.get_value::<i8>(verbosity);
        assert_eq!(-12, value.unwrap());         
    }

    #[test]
    fn mismatch_assignment() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let args = vec!["mismatch_assignment".to_string(),
            "-V=123.3".to_string()
        ];
        assert_eq!("IntegerCommandLineOptionType -V/--verbose: cannot set a floating point value to this option type".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn option_arg_already_assigned() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_version_option("print-out application version").unwrap();
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_integer_option('V', "verbose", false, "level", "set log verbosity level").unwrap();
        let _ = clp.add_short_string_option('c', false, "file path", "configuration file path").unwrap();
        let mut args = vec!["test_sho_help".to_string(), "-cV=goofy.txt".to_string()];        
        assert_eq!("IntegerCommandLineOptionType -V/--verbose: option argument already assigned".to_string(), 
                    clp.parse(&args).unwrap_err().to_string());
        args = vec!["test_sho_help".to_string(), "-cV".to_string(), "goofy.txt".to_string()];        
        assert_eq!("IntegerCommandLineOptionType -V/--verbose: option argument already assigned".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
        }

    #[test]
    fn test_boolean_option_eq() {
        let mut clp = CommandLineParser::new(None);
        let version_option = clp.add_version_option("print-out application version").unwrap();
        let args = vec!["test_boolean_option_eq".to_string(), "-v=false".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        assert_eq!(true, clp.is_set(&version_option));
        assert_eq!(Ok(false), clp.get_value::<bool>(version_option));
    }
    #[test]
    fn test_boolean_option_arg_already_assigned() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_help_option("print-out help menu").unwrap();
        let _ = clp.add_version_option("print-out application version").unwrap();
        let args = vec!["test_boolean_option_eq".to_string(), "-hv=false".to_string()];        
        assert_eq!("BooleanCommandLineOptionType -v/--version: option argument already assigned".to_string(), 
                clp.parse(&args).unwrap_err().to_string());
    }

    #[test]
    fn test_long_form_option() {
        let mut clp = CommandLineParser::new(None);
        let config_option = clp.add_long_string_option("config", false, "file path", "application configuration file").unwrap();
        let args = vec!["test_long_form_option".to_string(), "--config=goofy.properties".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        assert_eq!(String::from("goofy.properties"), clp.get_value::<String>(config_option).unwrap());
    }

    #[test]
    fn test_boolean_long_form_option() {
        let mut clp = CommandLineParser::new(None);
        let help_option = clp.add_help_option("show this help").unwrap();
        let args = vec!["test_boolean_long_form_option".to_string(), "--help=false".to_string()];        
        assert_eq!(Ok(()), clp.parse(&args));
        assert_eq!(Ok(false), clp.get_value::<bool>(help_option));
    }

    #[test]
    fn test_get_values() {
        let mut clp = CommandLineParser::new(None);
        let config_option = clp.add_string_option('c', "config", false, "file path", "application configuration file").unwrap();
        let _ = clp.add_integer_option('V', "verbosity", false, "level", "log verbosity level").unwrap();
        let args = vec!["test_get_values".to_string(), 
                                    "--config=config1.properties".to_string(),
                                    "--verbosity=2".to_string(), 
                                    "-c".to_string(), 
                                    "config2.properties".to_string(),
                                    "remaining args1".to_string(),
                                    "--config".to_string(),
                                    "config3.properties".to_string(),
                                    ];   
        assert_eq!(Ok(()), clp.parse(&args));
        assert_eq!(true, clp.is_set(&config_option));
        let values: Vec<String> = clp.get_values(config_option).unwrap();
        assert_eq!(vec!["config1.properties".to_string(), 
                        "config2.properties".to_string(),
                        "config3.properties".to_string()],
                    values);     
    }

    #[test]
    fn test_remaining_args() {
        let mut clp = CommandLineParser::new(None);
        let _ = clp.add_string_option('c', "config", false, "file path", "application configuration file").unwrap();
        let _ = clp.add_integer_option('V', "verbosity", false, "level", "log verbosity level").unwrap();
        let args = vec!["test_get_values".to_string(), 
                                    "--config=config1.properties".to_string(),
                                    "remaining_args1".to_string(),
                                    "--config".to_string(),
                                    "config2.properties".to_string(),
                                    "remaining_args2".to_string(),
                                    "--verbosity=2".to_string(), 
                                    "-c".to_string(), 
                                    "config3.properties".to_string(),
                                    "remaining_args3".to_string(),
                                    ];   
        assert_eq!(Ok(()), clp.parse(&args));
        let remaining_args = vec![
            "remaining_args1".to_string(),
            "remaining_args2".to_string(),
            "remaining_args3".to_string()
        ];
        assert_eq!(clp.get_remaining_args(), &remaining_args);
        println!("get_remaining_args(): {:#?}", clp.get_remaining_args());
        println!("remaining_args: {:#?}", remaining_args);
    }

    #[test]
    fn test_positional_args() {
        let mut clp = CommandLineParser::new(None);
        clp.set_parsing_mode(ParsingMode::PositionalArgumentsMode);
        let _ = clp.add_string_option('c', "config", false, "file path", "application configuration file").unwrap();
        let _ = clp.add_integer_option('V', "verbosity", false, "level", "log verbosity level").unwrap();

        let args = vec!["test_get_values".to_string(), 
                                    "--config=config1.properties".to_string(),
                                    "remaining_args1".to_string(),
                                    "--config".to_string(),
                                    "config2.properties".to_string(),
                                    "remaining_args2".to_string(),
                                    "--".to_string(),
                                    "--nocapture".to_string(), 
                                    "--test-threads=1".to_string()
                                    ];   
        assert_eq!(Ok(()), clp.parse(&args));
        let remaining_args = vec![
            "remaining_args1".to_string(),
            "remaining_args2".to_string(),
        ];
        assert_eq!(clp.get_remaining_args(), &remaining_args);
        println!("get_remaining_args(): {:#?}", clp.get_remaining_args());
        println!("remaining_args: {:#?}", remaining_args);

        let positional_args = vec![
            "--nocapture".to_string(), 
            "--test-threads=1".to_string()
        ];
        assert_eq!(clp.get_positional_args(), &positional_args);
        println!("get_positional_args(): {:#?}", clp.get_positional_args());
        println!("positional_args: {:#?}", positional_args);
    }

}
