use enum_primitive::FromPrimitive;

enum_from_primitive! {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub enum Key {
        Key0 = '0' as isize,
        Key1 = '1' as isize,
        Key2 = '2' as isize,
        Key3 = '3' as isize,
        Key4 = '4' as isize,
        Key5 = '5' as isize,
        Key6 = '6' as isize,
        Key7 = '7' as isize,
        Key8 = '8' as isize,
        Key9 = '9' as isize,

        A = 'A' as isize,
        B = 'B' as isize,
        C = 'C' as isize,
        D = 'D' as isize,
        E = 'E' as isize,
        F = 'F' as isize,
        G = 'G' as isize,
        H = 'H' as isize,
        I = 'I' as isize,
        J = 'J' as isize,
        K = 'K' as isize,
        L = 'L' as isize,
        M = 'M' as isize,
        N = 'N' as isize,
        O = 'O' as isize,
        P = 'P' as isize,
        Q = 'Q' as isize,
        R = 'R' as isize,
        S = 'S' as isize,
        T = 'T' as isize,
        U = 'U' as isize,
        V = 'V' as isize,
        W = 'W' as isize,
        X = 'X' as isize,
        Y = 'Y' as isize,
        Z = 'Z' as isize,

        Space = ' ' as isize,
        Tab = '\t' as isize,

        Backslash = '\\' as isize,
        Comma = ',' as isize,
        Equal = '=' as isize,
        LeftBracket = '[' as isize,
        Minus = '-' as isize,
        Period = '.' as isize,
        RightBracket = ']' as isize,
        Semicolon = ';' as isize,

        Slash = '/' as isize,
        Enter = '\n' as isize,

        Backspace = 128,
        Delete = 129,
        End = 130,

        F1 = 131,
        F2 = 132,
        F3 = 133,
        F4 = 134,
        F5 = 135,
        F6 = 136,
        F7 = 137,
        F8 = 138,
        F9 = 139,
        F10 = 140,
        F11 = 141,
        F12 = 142,
        F13 = 143,
        F14 = 144,
        F15 = 145,

        Down = 146,
        Left = 147,
        Right = 148,
        Up = 149,
        Apostrophe = 150,
        Backquote = 151,

        Escape = 152,

        Home = 153,
        Insert = 154,
        Menu = 155,

        PageDown = 156,
        PageUp = 157,

        Pause = 158,
        NumLock = 159,
        CapsLock = 160,
        ScrollLock = 161,
        LeftShift = 162,
        RightShift = 163,
        LeftCtrl = 164,
        RightCtrl = 165,

        NumPad0 = 166,
        NumPad1 = 167,
        NumPad2 = 168,
        NumPad3 = 169,
        NumPad4 = 170,
        NumPad5 = 171,
        NumPad6 = 172,
        NumPad7 = 173,
        NumPad8 = 174,
        NumPad9 = 175,
        NumPadDot = 176,
        NumPadSlash = 177,
        NumPadAsterisk = 178,
        NumPadMinus = 179,
        NumPadPlus = 180,
        NumPadEnter = 181,

        LeftAlt = 182,
        RightAlt = 183,

        LeftSuper = 184,
        RightSuper = 185,

        Unknown = 255,
    }
}

impl From<u8> for Key {
    fn from(val: u8) -> Key {
        match Key::from_i32(val as i32) {
            Some(key) => key,
            None => Key::Unknown,
        }
    }
}

impl Into<u8> for Key {
    fn into(self) -> u8 {
        (self as isize) as u8
    }
}