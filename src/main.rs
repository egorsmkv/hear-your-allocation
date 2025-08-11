use std::thread;
use std::time::Duration;

#[global_allocator]
static ALLOC: alloc_geiger::System = alloc_geiger::System::new();

fn simple_pattern() {
    // Define the rhythmic pattern. Each tuple represents an allocation "event".
    // The first element is the size of the allocation in kilobytes (KB).
    // The second element is the delay in milliseconds (ms) before the next event.
    // This example creates a simple "four-on-the-floor" beat with an accent.
    let pattern: Vec<(usize, u64)> = vec![
        // (Allocation Size in KB, Delay in ms)
        (2, 250), // Kick drum
        (1, 250), // Hi-hat
        (2, 250), // Snare
        (1, 250), // Hi-hat
        (4, 500), // Accentuated kick and a longer pause
        (1, 250),
        (2, 250),
        (1, 250),
    ];

    // A vector to hold the allocated data, preventing it from being
    // immediately deallocated and thus keeping the memory usage high.
    let mut allocated_data: Vec<Vec<u8>> = Vec::new();

    // The number of times to repeat the defined pattern.
    const REPETITIONS: usize = 10;

    for rep in 0..REPETITIONS {
        println!("\n--- Repetition {} of {} ---", rep + 1, REPETITIONS);

        for (size_kb, delay_ms) in &pattern {
            // Simulate a heap allocation of a specified size.
            let size_bytes = size_kb * 1024;
            let new_allocation: Vec<u8> = vec![0; size_bytes];
            allocated_data.push(new_allocation);

            println!("Allocated {} KB...", size_kb);

            // Add the specified delay before the next allocation.
            thread::sleep(Duration::from_millis(*delay_ms));
        }
    }
}

fn harder_pattern() {
    println!("The pattern of allocations can be edited below to create different sounds.");

    // Define the rhythmic pattern. Each tuple represents an allocation "event".
    // The first element is the size of the allocation in kilobytes (KB).
    // The second element is the delay in milliseconds (ms) before the next event.
    // This example now creates a longer, more varied musical phrase.
    let pattern: Vec<(usize, u64)> = vec![
        // (Allocation Size in KB, Delay in ms)
        // Part 1: A short, staccato phrase
        (2, 250),
        (1, 125),
        (1, 125),
        (2, 250),
        (4, 500),
        // Part 2: A slower, more melodic section
        (1, 500),
        (2, 250),
        (3, 250),
        (4, 500),
        // Part 3: A faster, more complex part
        (2, 125),
        (1, 125),
        (2, 125),
        (1, 125),
        (3, 250),
        (2, 250),
        (4, 500),
        // Part 4: A long, sustained note with a pause
        (6, 1000),
        (0, 1000), // A 'rest' or silent delay
    ];

    // A vector to hold the allocated data, preventing it from being
    // immediately deallocated and thus keeping the memory usage high.
    let mut allocated_data: Vec<Vec<u8>> = Vec::new();

    // The number of times to repeat the defined pattern.
    const REPETITIONS: usize = 2;

    for rep in 0..REPETITIONS {
        println!("\n--- Repetition {} of {} ---", rep + 1, REPETITIONS);

        for (size_kb, delay_ms) in &pattern {
            // Check for a 'rest' (size_kb == 0) and skip allocation.
            if *size_kb > 0 {
                // Simulate a heap allocation of a specified size.
                let size_bytes = size_kb * 1024;
                let new_allocation: Vec<u8> = vec![0; size_bytes];
                allocated_data.push(new_allocation);
                println!("Allocated {} KB...", size_kb);
            } else {
                println!("Resting...");
            }

            // Add the specified delay before the next allocation.
            thread::sleep(Duration::from_millis(*delay_ms));
        }
    }

    println!("\nAllocation pattern finished.");
}

fn ukrainian_anthem() {
    println!("The pattern is designed to mimic the Ukrainian national anthem's rhythm.");

    // Define the rhythmic pattern based on the opening phrase of the anthem.
    // Each tuple represents an allocation "event".
    // The first element is the size of the allocation in kilobytes (KB).
    // The second element is the delay in milliseconds (ms) before the next event.
    let pattern: Vec<(usize, u64)> = vec![
        // (Allocation Size in KB, Delay in ms)
        // Ще не вмерла України ні слава, ні воля...
        (2, 250), // Ще
        (2, 250), // не
        (4, 500), // вмер-
        (2, 250), // -ла
        (4, 500), // У-
        (3, 250), // кра-
        (3, 250), // -ї-
        (5, 750), // -ни
        (0, 500), // rest
        // ...ні слава, ні воля.
        (2, 250),  // ні
        (4, 500),  // сла-
        (2, 250),  // -ва,
        (2, 250),  // ні
        (4, 500),  // во-
        (6, 1000), // -ля.
        (0, 1000), // long rest
    ];

    // A vector to hold the allocated data, preventing it from being
    // immediately deallocated and thus keeping the memory usage high.
    let mut allocated_data: Vec<Vec<u8>> = Vec::new();

    // The number of times to repeat the defined pattern.
    const REPETITIONS: usize = 3;

    for rep in 0..REPETITIONS {
        println!("\n--- Repetition {} of {} ---", rep + 1, REPETITIONS);

        for (size_kb, delay_ms) in &pattern {
            // Check for a 'rest' (size_kb == 0) and skip allocation.
            if *size_kb > 0 {
                // Simulate a heap allocation of a specified size.
                let size_bytes = size_kb * 1024;
                let new_allocation: Vec<u8> = vec![0; size_bytes];
                allocated_data.push(new_allocation);
                println!("Allocated {} KB...", size_kb);
            } else {
                println!("Resting...");
            }

            // Add the specified delay before the next allocation.
            thread::sleep(Duration::from_millis(*delay_ms));
        }
    }

    println!("\nAllocation pattern finished.");
}

fn main() {
    simple_pattern();

    thread::sleep(Duration::from_millis(1000));

    harder_pattern();

    thread::sleep(Duration::from_millis(1000));

    ukrainian_anthem();
}
