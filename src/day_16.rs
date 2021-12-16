use crate::helpers::read_lines;

struct State {
    version_sum: usize
}

pub fn day_16_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_16.txt") {
        
        for file_line in file_lines {
            let line = file_line.unwrap();
            
            let binary = convert_to_binary_from_hex(&line);

            println!("{}", binary);

            let mut state = State{ version_sum: 0 };

            let result = read_packet(&binary, &mut state);

            println!("Sum of all version numbers = {}", state.version_sum);
            println!("Result of all operators = {}", result.1);
        }
    }
}

fn read_packet(input: &str, state: &mut State) -> (usize, usize) {
    println!("---- Begin reading packet ----");
    println!("Packet binary = {}", input);

    let mut operator_result = 0;

    let mut pointer = 0;
    let packet_version_bin = &input[pointer..pointer+3];
    let packet_version = usize::from_str_radix(packet_version_bin, 2).unwrap();
    println!("packet_version = {} = {}", packet_version_bin, packet_version);

    state.version_sum += packet_version;

    pointer += 3;

    let packet_type_bin = &input[pointer..pointer+3];
    let packet_type = usize::from_str_radix(packet_type_bin, 2).unwrap();
    println!("packet_type = {} = {}", packet_type_bin, packet_type);

    pointer += 3;

    if packet_type == 4 {
        // Literal value packet type

        let mut packet_value_bin = String::new();
        let mut last_group = false;
        while !last_group {
            last_group = usize::from_str_radix(&input[pointer..pointer+1], 2).unwrap() == 0;

            pointer += 1;

            let group_value_bin = &input[pointer..pointer+4];
            packet_value_bin.push_str(group_value_bin);

            pointer += 4;
        }

        let packet_value = usize::from_str_radix(&packet_value_bin, 2).unwrap();
        println!("Packet type 4 binary value = {} = {}", packet_value_bin, packet_value);

        operator_result = packet_value;
    } else {
        // Operator packet type

        let mut sub_packet_results = Vec::<usize>::new();
        
        let packet_length_type = &input[pointer..pointer+1];
        pointer += 1;
        println!("Operator packet type {} - length type = {}", packet_type, packet_length_type);

        if packet_length_type == "0" {
            // Next 15 bits represents the total length in bits of all sub-packets
            let total_sub_packet_length_bin = &input[pointer..pointer+15];
            let total_sub_packet_length = usize::from_str_radix(&total_sub_packet_length_bin, 2).unwrap();

            println!("Total sub packet length = {} = {}", total_sub_packet_length_bin, total_sub_packet_length);

            pointer += 15;

            let mut length_remaining = total_sub_packet_length as usize;

            while length_remaining > 0 {
                let (sub_packet_length, packet_result) = read_packet(&input[pointer..pointer+length_remaining], state);

                pointer += sub_packet_length;
                length_remaining -= sub_packet_length;

                sub_packet_results.push(packet_result);
            }

        } else if packet_length_type == "1" {
            // Next 11 bits represents numbers of sub-packets in this packet
            let number_of_sub_packets_bin = &input[pointer..pointer+11];
            let number_of_sub_packets = usize::from_str_radix(&number_of_sub_packets_bin, 2).unwrap();

            println!("Number of sub packets = {} = {}", number_of_sub_packets_bin, number_of_sub_packets);

            pointer += 11;

            let mut sub_packets_remaining = number_of_sub_packets as usize;

            while sub_packets_remaining > 0 {
                let (sub_packet_length, packet_result) = read_packet(&input[pointer..], state);

                pointer += sub_packet_length;
                sub_packets_remaining -= 1;

                sub_packet_results.push(packet_result);
            }
        } else {
            panic!();
        }

        if packet_type == 0 {
            // Sum
            operator_result = sub_packet_results.iter().sum();
        } else if packet_type == 1 {
            // Product
            operator_result = sub_packet_results[0];
            for i in 1..sub_packet_results.len() {
                operator_result *= sub_packet_results[i];
            }
        } else if packet_type == 2 {
            // Minimum
            operator_result = *sub_packet_results.iter().min().unwrap();
        } else if packet_type == 3 {
            // Maximum
            operator_result = *sub_packet_results.iter().max().unwrap();
        } else if packet_type == 5 {
            // Greater than
            if sub_packet_results[0] > sub_packet_results[1] {
                operator_result = 1;
            } else {
                operator_result = 0;
            }
        } else if packet_type == 6 {
            // Less than
            if sub_packet_results[0] < sub_packet_results[1] {
                operator_result = 1;
            } else {
                operator_result = 0;
            }
        } else if packet_type == 7 {
            // Equal to
            if sub_packet_results[0] == sub_packet_results[1] {
                operator_result = 1;
            } else {
                operator_result = 0;
            }
        }
    }

    println!("---- Ended reading packet with length {} and value {} ----", pointer, operator_result);

    (pointer, operator_result)
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
