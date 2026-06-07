package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.domain.repository.UserRepository;
import jakarta.transaction.Transactional;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

import java.time.LocalDateTime;

@Service
public class LoginAttemptService {

    private static final Logger log = LoggerFactory.getLogger(LoginAttemptService.class);

    @Value("${app.security.max-login-attempts}")
    private int maxAttempts;

    @Value("${app.security.lockout-duration-minutes}")
    private int lockoutMinutes;

    private final UserRepository userRepository;

    public LoginAttemptService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    @Transactional
    public void onSuccess(String email) {
        userRepository.findByEmail(email).ifPresent(user -> {
            if (user.getFailedLoginAttempts() > 0 || user.getLockedUntil() != null) {
                userRepository.save(user.resetFailedAttempts());
                log.debug("Reset failed login attempts for {}", email);
            }
        });
    }

    @Transactional
    public void onFailure(String email) {
        userRepository.findByEmail(email).ifPresent(user -> {
            var updated = user.incrementFailedAttempts();
            if (updated.getFailedLoginAttempts() >= maxAttempts) {
                updated = updated.lockUntil(LocalDateTime.now().plusMinutes(lockoutMinutes));
                log.warn("Account locked for {} until {}", email, updated.getLockedUntil());
            } else {
                log.warn("Failed login attempt {}/{} for {}",
                        updated.getFailedLoginAttempts(), maxAttempts, email);
            }
            userRepository.save(updated);
        });
    }
}