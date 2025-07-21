use email_syntax_verify_opt::ValidateEmail;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_test_high_volume_validation() {
        let test_emails = vec![
            "user1@example.com",
            "user2@domain.co.uk",
            "test.email@subdomain.example.org",
            "invalid.email",
            "@invalid.com",
            "user@",
            "user@[127.0.0.1]",
            "user+tag@example.com",
            "user_name@example.com",
            "user-name@example.com",
        ];

        let iterations = 100_000;
        let start = Instant::now();

        for i in 0..iterations {
            let email = &test_emails[i % test_emails.len()];
            let _ = email.validate_email();
        }

        let duration = start.elapsed();
        let emails_per_second = iterations as f64 / duration.as_secs_f64();

        println!("Processed {} emails in {:?}", iterations, duration);
        println!("Rate: {:.0} emails/second", emails_per_second);

        assert!(
            emails_per_second > 100_000.0,
            "Should process at least 100k emails/second, got: {:.0}",
            emails_per_second
        );
    }

    #[test]
    fn stress_test_concurrent_validation() {
        let test_emails = Arc::new(vec![
            "user1@example.com",
            "user2@domain.co.uk",
            "test.email@subdomain.example.org",
            "invalid.email",
            "@invalid.com",
            "user@",
            "user@[127.0.0.1]",
            "user+tag@example.com",
        ]);

        let num_threads = 8;
        let iterations_per_thread = 10_000;
        let results = Arc::new(Mutex::new(Vec::new()));

        let start = Instant::now();

        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                let emails = Arc::clone(&test_emails);
                let results = Arc::clone(&results);

                thread::spawn(move || {
                    let thread_start = Instant::now();
                    let mut local_results = Vec::new();

                    for i in 0..iterations_per_thread {
                        let email = &emails[i % emails.len()];
                        let result = email.validate_email();
                        local_results.push((thread_id, i, result));
                    }

                    let thread_duration = thread_start.elapsed();

                    {
                        let mut shared_results = results.lock().unwrap();
                        shared_results.extend(local_results);
                    }

                    thread_duration
                })
            })
            .collect();

        let thread_durations: Vec<Duration> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();

        let total_duration = start.elapsed();
        let total_validations = num_threads * iterations_per_thread;
        let validations_per_second = total_validations as f64 / total_duration.as_secs_f64();

        println!(
            "Concurrent test: {} threads, {} validations each",
            num_threads, iterations_per_thread
        );
        println!("Total time: {:?}", total_duration);
        println!("Rate: {:.0} validations/second", validations_per_second);

        for (i, duration) in thread_durations.iter().enumerate() {
            println!("Thread {}: {:?}", i, duration);
        }

        assert!(
            validations_per_second > 50_000.0,
            "Concurrent validation should achieve at least 50k validations/second, got: {:.0}",
            validations_per_second
        );

        let results_guard = results.lock().unwrap();
        assert_eq!(
            results_guard.len(),
            total_validations,
            "Should have results for all validations"
        );
    }

    #[test]
    fn stress_test_memory_usage() {
        let base_email = "user@example.com";
        let iterations = 1_000_000;

        let start_memory = get_memory_usage();
        let start_time = Instant::now();

        for _ in 0..iterations {
            let _ = base_email.validate_email();
        }

        let end_time = Instant::now();
        let end_memory = get_memory_usage();

        let duration = end_time - start_time;
        let memory_increase = end_memory.saturating_sub(start_memory);

        println!("Memory test: {} iterations in {:?}", iterations, duration);
        println!("Memory increase: {} bytes", memory_increase);

        assert!(
            memory_increase < 1024 * 1024,
            "Memory increase should be less than 1MB, got: {} bytes",
            memory_increase
        );
    }

    #[test]
    fn stress_test_long_emails() {
        let long_emails = vec![
            format!("{}@{}.com", "a".repeat(60), "b".repeat(200)),
            format!(
                "{}@{}.co.uk",
                "user.name.with.dots".repeat(3),
                "very-long-domain-name".repeat(8)
            ),
            format!("{}@{}.org", "x".repeat(64), "y".repeat(240)),
        ];

        let iterations = 10_000;
        let start = Instant::now();

        for i in 0..iterations {
            let email = &long_emails[i % long_emails.len()];
            let _ = email.validate_email();
        }

        let duration = start.elapsed();

        println!(
            "Long email test: {} iterations in {:?}",
            iterations, duration
        );

        assert!(
            duration < Duration::from_millis(1000),
            "Long email validation should complete in <1s, took: {:?}",
            duration
        );
    }

    #[test]
    fn stress_test_pathological_cases() {
        let pathological_cases = vec![
            "@".repeat(1000),
            ".".repeat(1000),
            "a".repeat(1000) + "@" + &"b".repeat(1000) + ".com",
            format!("{}@{}", "user", ".".repeat(500) + "com"),
            format!("{}@{}", ".".repeat(500) + "user", "example.com"),
            format!("user@{}com", "a.".repeat(500)),
        ];

        for (i, email) in pathological_cases.iter().enumerate() {
            let start = Instant::now();
            let result = email.validate_email();
            let duration = start.elapsed();

            println!(
                "Pathological case {}: {} chars, result: {}, time: {:?}",
                i,
                email.len(),
                result,
                duration
            );

            assert!(
                duration < Duration::from_millis(10),
                "Pathological case {} should complete in <10ms, took: {:?}",
                i,
                duration
            );
        }
    }

    #[test]
    fn stress_test_repeated_validation() {
        let email = "test@example.com";
        let iterations = 10_000_000;

        let start = Instant::now();

        for _ in 0..iterations {
            let result = email.validate_email();
            assert!(result, "Email should remain valid");
        }

        let duration = start.elapsed();
        let validations_per_second = iterations as f64 / duration.as_secs_f64();

        println!(
            "Repeated validation: {} iterations in {:?}",
            iterations, duration
        );
        println!("Rate: {:.0} validations/second", validations_per_second);

        assert!(
            validations_per_second > 1_000_000.0,
            "Should achieve at least 1M validations/second for simple case, got: {:.0}",
            validations_per_second
        );
    }

    #[test]
    fn stress_test_string_allocation_patterns() {
        let base_patterns = [
            "user@example.com",
            "test.email@domain.co.uk",
            "user+tag@subdomain.example.org",
        ];

        let iterations = 100_000;
        let start = Instant::now();

        for i in 0..iterations {
            let pattern = base_patterns[i % base_patterns.len()];

            let owned_string = String::from(pattern);
            let _ = owned_string.validate_email();

            let str_slice = pattern;
            let _ = str_slice.validate_email();

            let byte_slice = pattern.as_bytes();
            let _ = byte_slice.validate_email();
        }

        let duration = start.elapsed();

        println!(
            "String allocation test: {} iterations in {:?}",
            iterations, duration
        );

        assert!(
            duration < Duration::from_millis(5000),
            "String allocation test should complete in <5s, took: {:?}",
            duration
        );
    }

    pub fn get_memory_usage() -> usize {
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            if let Ok(status) = fs::read_to_string("/proc/self/status") {
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<usize>() {
                                return kb * 1024;
                            }
                        }
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod endurance_tests {
    use super::*;

    #[test]
    fn endurance_test_continuous_validation() {
        let test_emails = [
            "user1@example.com",
            "user2@domain.co.uk",
            "invalid.email",
            "user@[127.0.0.1]",
            "test@subdomain.example.org",
        ];

        let test_duration = Duration::from_secs(10);
        let start = Instant::now();
        let mut iteration_count = 0;
        let mut error_count = 0;

        while start.elapsed() < test_duration {
            for email in &test_emails {
                let result = std::panic::catch_unwind(|| email.validate_email());

                match result {
                    Ok(_) => iteration_count += 1,
                    Err(_) => error_count += 1,
                }
            }
        }

        let actual_duration = start.elapsed();
        let validations_per_second = iteration_count as f64 / actual_duration.as_secs_f64();

        println!(
            "Endurance test: {} validations in {:?}",
            iteration_count, actual_duration
        );
        println!("Rate: {:.0} validations/second", validations_per_second);
        println!("Errors: {}", error_count);

        assert_eq!(
            error_count, 0,
            "No errors should occur during endurance test"
        );
        assert!(
            validations_per_second > 10_000.0,
            "Should maintain at least 10k validations/second, got: {:.0}",
            validations_per_second
        );
    }

    #[test]
    fn endurance_test_memory_stability() {
        let email = "test@example.com";
        let check_interval = 100_000;
        let total_iterations = 1_000_000;

        let mut memory_readings = Vec::new();

        for i in 0..total_iterations {
            let _ = email.validate_email();

            if i % check_interval == 0 {
                let memory = crate::stress_tests::get_memory_usage();
                memory_readings.push(memory);

                if memory_readings.len() > 1 {
                    let previous = memory_readings[memory_readings.len() - 2];
                    let current = memory_readings[memory_readings.len() - 1];
                    let increase = current.saturating_sub(previous);

                    println!(
                        "Iteration {}: Memory usage: {} bytes (+{})",
                        i, current, increase
                    );

                    assert!(
                        increase < 1024 * 1024,
                        "Memory increase should be <1MB per {}k iterations, got: {} bytes",
                        check_interval / 1000,
                        increase
                    );
                }
            }
        }

        if memory_readings.len() >= 2 {
            let initial = memory_readings[0];
            let final_mem = memory_readings[memory_readings.len() - 1];
            let total_increase = final_mem.saturating_sub(initial);

            println!("Total memory increase: {} bytes", total_increase);

            assert!(
                total_increase < 10 * 1024 * 1024,
                "Total memory increase should be <10MB, got: {} bytes",
                total_increase
            );
        }
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;

    #[test]
    fn load_test_burst_validation() {
        let emails: Vec<String> = (0..1000)
            .map(|i| format!("user{}@example{}.com", i, i % 100))
            .collect();

        let burst_size = 10_000;
        let num_bursts = 10;

        for burst in 0..num_bursts {
            let start = Instant::now();

            for i in 0..burst_size {
                let email = &emails[i % emails.len()];
                let _ = email.validate_email();
            }

            let duration = start.elapsed();
            let rate = burst_size as f64 / duration.as_secs_f64();

            println!(
                "Burst {}: {} validations in {:?} ({:.0}/sec)",
                burst, burst_size, duration, rate
            );

            assert!(
                rate > 50_000.0,
                "Burst {} should achieve >50k validations/second, got: {:.0}",
                burst,
                rate
            );

            thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn load_test_sustained_throughput() {
        let emails = vec![
            "user@example.com",
            "test@domain.co.uk",
            "invalid.email",
            "user@[192.168.1.1]",
            "complex.email+tag@subdomain.example.org",
        ];

        let test_duration = Duration::from_secs(30);
        let sample_interval = Duration::from_secs(1);

        let start = Instant::now();
        let mut samples = Vec::new();
        let mut total_validations = 0;
        let mut last_sample_time = start;
        let mut last_sample_count = 0;

        while start.elapsed() < test_duration {
            let email = &emails[total_validations % emails.len()];
            let _ = email.validate_email();
            total_validations += 1;

            if last_sample_time.elapsed() >= sample_interval {
                let current_time = Instant::now();
                let sample_duration = current_time - last_sample_time;
                let sample_count = total_validations - last_sample_count;
                let sample_rate = sample_count as f64 / sample_duration.as_secs_f64();

                samples.push(sample_rate);

                println!(
                    "Sample {}: {:.0} validations/second",
                    samples.len(),
                    sample_rate
                );

                last_sample_time = current_time;
                last_sample_count = total_validations;
            }
        }

        let total_duration = start.elapsed();
        let average_rate = total_validations as f64 / total_duration.as_secs_f64();

        let min_rate = samples.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_rate = samples.iter().fold(0.0f64, |a, &b| a.max(b));
        let rate_variance = samples
            .iter()
            .map(|&rate| (rate - average_rate).powi(2))
            .sum::<f64>()
            / samples.len() as f64;

        println!("Sustained load test results:");
        println!("Total validations: {}", total_validations);
        println!("Average rate: {:.0}/sec", average_rate);
        println!("Min rate: {:.0}/sec", min_rate);
        println!("Max rate: {:.0}/sec", max_rate);
        println!("Rate variance: {:.0}", rate_variance);

        assert!(
            average_rate > 100_000.0,
            "Average rate should be >100k/sec, got: {:.0}",
            average_rate
        );

        assert!(
            min_rate > 50_000.0,
            "Minimum rate should be >50k/sec, got: {:.0}",
            min_rate
        );

        assert!(
            rate_variance < (average_rate * 0.1).powi(2),
            "Rate variance should be <10% of average, got: {:.0}",
            rate_variance.sqrt()
        );
    }
}
