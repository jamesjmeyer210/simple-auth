mod problem_details;
mod role_dto;
mod user_dto;
pub(crate) mod oauth;

pub type ProblemDetails<'p> = problem_details::ProblemDetails<'p>;
pub(crate) type AddRoleDto = role_dto::AddRoleDto;
pub(crate) type AddUserDto = user_dto::AddUserDto;
pub(crate) type AddUserContactDto = user_dto::AddUserContactDto;