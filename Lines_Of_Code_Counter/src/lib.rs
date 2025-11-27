mod line_utils;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = line_utils::determine_if_line_is_source("// comment");
        assert_eq!(result, true);
    }
}
