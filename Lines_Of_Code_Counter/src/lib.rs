mod line_utils;
mod file_traversal_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = line_utils::determine_if_line_is_source("// comment");
        assert_eq!(result, true);
    }


    #[test]
    fn should_file_be_processed() {
        let should_be_processed = file_traversal_utils::should_file_be_processed("Den.Cpp");

        assert_eq!(should_be_processed.0, true);


    }
}
