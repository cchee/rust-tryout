pub mod response {
    #![allow(non_snake_case)]

    use utoipa::ToSchema;

    use crate::cost_items::CostItems;

    #[derive(ToSchema)]
    pub struct CostItemsResponse {
        pub Ok: Vec<CostItems>,
    }
    #[derive(ToSchema)]
    pub struct CostItemResponse {
        pub Ok: CostItems,
    }
    #[derive(ToSchema)]
    pub struct DeleteResponse {
        pub deleted: usize,
    }
    #[derive(ToSchema)]
    pub struct ErrorResponse {
        pub Err: String,
    }
}

pub mod check {
    use std::collections::HashMap;

    use crate::error_handler::CustomError;

    /// Check if a &str is a int number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_long("2") {
    ///     Ok(n) => assert_eq!(2, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    /// ```
    /// use lib_api::utils::check;
    ///
    /// match check::validate_long("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_long(int_str: &str) -> Result<i64, CustomError> {
        int_str.parse::<i64>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{int_str}', not a valid integer"),
            )
        })
    }

    pub fn validate_int(int_str: &str) -> Result<i32, CustomError> {
        int_str.parse::<i32>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{int_str}', not a valid integer"),
            )
        })
    }

    /// Check if a &str is a float number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("1.1") {
    ///     Ok(n) => assert_eq!(1.1, n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::validate_float("a") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid float" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    ///```
    pub fn validate_float(float_str: &str) -> Result<f64, CustomError> {
        float_str.parse::<f64>().map_err(|_| {
            CustomError::new(
                400,
                format!("Error parsing string: '{float_str}', not a valid float"),
            )
        })
    }

    /// Check if a all items of &str comma separated items its a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::parse_ids("1,2") {
    ///     Ok(vec_n) => assert_eq!(vec![1, 2], vec_n),
    ///     Err(e) => panic!("Returned Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// match check::parse_ids("a,2") {
    ///     Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    /// ```
    pub fn parse_ids(ids_str: &str) -> Result<Vec<i64>, CustomError> {
        let ids: Vec<i64> = ids_str
            .split(',')
            .map(validate_long)
            .collect::<Result<Vec<i64>, CustomError>>()?;
        Ok(ids)
    }

    /// Check if a params for cost item are correct.
    ///
    /// pub struct CostItem {
    ///     pub id: i64,
    ///     pub name: String,
    ///     pub price: BigDecimal,
    ///     pub notes: String,
    /// }
    ///
    /// # Examples
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("name".to_string(), "English Congregation".to_string());
    /// params.insert("price".to_string(), "1.20".to_string());
    /// params.insert("notes".to_string(), "1112223333".to_string());
    ///
    /// match check::validate_cost_item_params(&params) {
    ///     Ok(..) => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    /// }
    /// ```
    ///
    /// ```
    /// use lib_api::utils::check;
    /// use std::collections::HashMap;
    ///
    /// let mut params = HashMap::new();
    /// params.insert("ids".to_string(), "1,2".to_string());
    /// params.insert("id".to_string(), "1".to_string());
    /// params.insert("name".to_string(), "English Congregation".to_string());
    /// params.insert("price".to_string(), "1.20".to_string());
    /// params.insert("notes".to_string(), "1112223333".to_string());
    ///
    /// match check::validate_cost_item_params(&params) {
    ///     Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
    ///     Err(e) => panic!("Returned incorrect Err! => {e}"),
    ///     Ok(_) => panic!("Returned an Ok variant!"),
    /// }
    /// ```
    pub fn validate_cost_item_params(params: &HashMap<String, String>) -> Result<bool, CustomError> {
        let keys = vec![
            "id".to_string(),
            "ids".to_string(),
            "name".to_string(),
            "price".to_string(),
            "notes".to_string(),
        ];

        for key in params.keys() {
            if !keys.contains(key) {
                return Err(CustomError::new(
                    400,
                    format!("the parameter '{key}' is incorrect"),
                ));
            }
        }

        if params.get("id").is_some() && params.get("ids").is_some() {
            return Err(CustomError::new(
                400,
                "select only one of them, id xor ids".to_string(),
            ));
        }

        if let Some(id) = params.get("id") {
            match validate_long(id) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        if let Some(ids) = params.get("ids") {
            match parse_ids(ids) {
                Ok(..) => (),
                Err(err) => return Err(err),
            }
        }

        Ok(true)
    }
}

// // Alternative tests
// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;
//
//     use crate::utils::check;
//
//     #[test]
//     fn validate_long_test_ok() {
//         match check::validate_long("2") {
//             Ok(n) => assert_eq!(2, n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_long_test_ko() {
//         match check::validate_long("a") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_float_test_ok() {
//         match check::validate_float("1.1") {
//             Ok(n) => assert_eq!(1.1, n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_float_test_ko() {
//         match check::validate_float("a") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid float" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_ids_test_ok() {
//         match check::parse_ids("1,2") {
//             Ok(vec_n) => assert_eq!(vec![1, 2], vec_n),
//             Err(e) => panic!("Returned Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_ids_test_ko() {
//         match check::parse_ids("a,1") {
//             Err(e) if e.to_string() == "Error parsing string: 'a', not a valid integer" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
//
//     #[test]
//     fn validate_cost_item_params_test_ok() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("name".to_string(), "English Congregation".to_string());
//         params.insert("price".to_string(), "1.20".to_string());
//         params.insert("notes".to_string(), "1112223333".to_string());
//
//         match check::validate_cell_group_params(&params) {
//             Ok(..) => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//         }
//     }
//
//     #[test]
//     fn validate_cost_item_params_test_ko_ids() {
//         let mut params = HashMap::new();
//         params.insert("id".to_string(), "1".to_string());
//         params.insert("ids".to_string(), "1,2".to_string());
//         params.insert("name".to_string(), "English Congregation".to_string());
//         params.insert("price".to_string(), "1.20".to_string());
//         params.insert("notes".to_string(), "1112223333".to_string());
//
//         match check::validate_cost_item_params(&params) {
//             Err(e) if e.to_string() == "select only one of them, id xor ids" => (),
//             Err(e) => panic!("Returned incorrect Err! => {e}"),
//             Ok(_) => panic!("Returned an Ok variant!"),
//         }
//     }
// }