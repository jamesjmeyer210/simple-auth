mod base64_str;
mod jwt_str;

pub type JwtStr<'s> = jwt_str::JwtStr<'s>;
pub type JwtStrParts<'s> = jwt_str::JwtStrParts<'s>;
pub type JwtByteParts = jwt_str::JwtByteParts;