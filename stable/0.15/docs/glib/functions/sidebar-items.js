initSidebarItems({"fn":[["access",""],["application_name",""],["base64_decode",""],["base64_encode",""],["check_version",""],["codeset",""],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["console_charset",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ",""],["file_set_contents",""],["file_set_contents_full",""],["filename_display_basename",""],["filename_display_name",""],["format_size",""],["format_size_full",""],["host_name",""],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["language_names",""],["language_names_with_category",""],["listenv",""],["locale_variants",""],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp_full",""],["monotonic_time",""],["num_processors",""],["on_error_query",""],["on_error_stack_trace",""],["os_info",""],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["real_time",""],["reload_user_special_dirs_cache",""],["set_application_name",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest",""],["spawn_async",""],["spawn_check_exit_status","An old name for [`spawn_check_wait_status()`][crate::spawn_check_wait_status()], deprecated because its name is misleading."],["spawn_check_wait_status","Set `error` if `wait_status` indicates the child exited abnormally (e.g. with a nonzero exit code, or via a fatal signal)."],["spawn_command_line_async","A simple version of [`spawn_async()`][crate::spawn_async()] that parses a command line with [`shell_parse_argv()`][crate::shell_parse_argv()] and passes it to [`spawn_async()`][crate::spawn_async()]."],["system_config_dirs",""],["system_data_dirs",""],["unicode_script_from_iso15924","Looks up the Unicode script for `iso15924`. ISO 15924 assigns four-letter codes to scripts. For example, the code for Arabic is ‘Arab’. This function accepts four letter codes encoded as a `guint32` in a big-endian fashion. That is, the code expected for Arabic is 0x41726162 (0x41 is ASCII code for ‘A’, 0x72 is ASCII code for ‘r’, etc)."],["unicode_script_to_iso15924","Looks up the ISO 15924 code for `script`. ISO 15924 assigns four-letter codes to scripts. For example, the code for Arabic is ‘Arab’. The four letter codes are encoded as a `guint32` by this function in a big-endian fashion. That is, the code returned for Arabic is 0x41726162 (0x41 is ASCII code for ‘A’, 0x72 is ASCII code for ‘r’, etc)."],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["user_cache_dir",""],["user_config_dir",""],["user_data_dir",""],["user_runtime_dir",""],["user_special_dir",""],["user_state_dir",""],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."]]});