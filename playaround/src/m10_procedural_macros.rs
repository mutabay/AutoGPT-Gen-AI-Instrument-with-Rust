


#[cfg(test)]
mod tests {
    use my_proc_macro::function_to_string;
    use ai_functions::ai_function;
     
    #[ai_function]
    fn another_ai_function(_whatever_param: &str) -> String {
        /// This is an awesome function, from the crates.io libraries
        /// We shall give it to an AI to guess and output
        /// In a structured manner
        println!("{}", _OUTPUT);
    }

    #[function_to_string]
    fn some_function_for_ai_llm(_whatever_param: &str) {
        /// This is an awesome function
        /// We shall give it to an AI to guess and output
        /// In a structured manner
        println!("{}", _OUTPUT);
    }
   
   #[test]
   fn test_procedural_macros() {
       // Test the ai_function macro
       let ai_output = another_ai_function("test input");
       println!("AI Function Output: {}", ai_output);
       
       // Test the function_to_string macro
       let function_output = some_function_for_ai_llm("test input");
       println!("Function Output: {}", function_output);
   }
}