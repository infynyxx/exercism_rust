
pub fn lsp(input: &str, digits: usize) -> Result<i32, &str> {
    if input.is_empty() && digits == 0 {
        return Ok(1);
    }

    if !input.is_empty() && digits ==  0 {
        return Ok(1);
    }

    if  digits > input.len() {
        return Err("span is longer than number");
    }

    let mut vec = Vec::with_capacity(input.len());
    let mut iter = input.chars().into_iter();
    loop {
        match iter.next() {
            Some(x) => {
                let to_digit = x.to_digit(10);
                if to_digit.is_none() {
                    return Err("non digits");
                } else {
                    vec.push(to_digit.unwrap() as i32);
                }
            }
            None => break,
        }
    }

    let mut highest: i32 = 0;

    for i in 0..vec.len() {
        let last_index = digits + i;
        if last_index > vec.len() {
            break;
        }
        let mut product: i32 = 0;
        for j in i..last_index {
            let new_product =  if j == i {
                 vec[j]
            } else {
                product * vec[j]
            };
            product = new_product;
        }
        if product > highest {
            highest = product;
        }
    }
    Ok(highest)
}
