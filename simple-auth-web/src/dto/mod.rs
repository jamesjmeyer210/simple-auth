mod problem_details;
mod role_dto;

pub(crate) type ProblemDetails<'p> = problem_details::ProblemDetails<'p>;
pub(crate) type AddRoleDto = role_dto::AddRoleDto;