use crate::common::subject_generator::SubjectGenerator;
use crate::random_subject_generator::RandomSubjectGenerator;

pub fn subject_gen_factory() -> impl SubjectGenerator {
    RandomSubjectGenerator::new()
}