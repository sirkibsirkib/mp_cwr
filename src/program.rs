use seaso::{
    dynamics::{Denotation, Knowledge},
    statics::ExecutableError,
    DomainId, ExecutableConfig, PartName, Program,
};

pub enum WfErr<'a> {
    RepeatedlyDefinedPart(&'a PartName),
    ExecutableError(ExecutableError<'a>),
    DomainCycle(DomainId),
}

trait ErrOr<O> {
    type Out;
    fn err_or(self, o: O) -> Self::Out;
}
impl<E, O> ErrOr<O> for Option<E> {
    type Out = Result<O, E>;
    fn err_or(self, o: O) -> Result<O, E> {
        match self {
            Some(e) => Err(e),
            None => Ok(o),
        }
    }
}

pub fn well_founded_denotation(program: &Program) -> Result<Denotation<Knowledge>, WfErr> {
    program.repeatedly_defined_part().err_or(()).map_err(WfErr::RepeatedlyDefinedPart)?;
    let ep = program.executable(ExecutableConfig::default()).map_err(WfErr::ExecutableError)?;
    ep.unbounded_domain_cycle().err_or(()).map_err(Clone::clone).map_err(WfErr::DomainCycle)?;
    Ok(ep.denotation().denotation)
}
