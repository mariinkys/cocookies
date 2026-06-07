package dev.mariinkys.cocookies.domain.models;

import java.time.LocalDateTime;
import java.util.UUID;

public class User {

    private final UUID id;
    private final String name;
    private final String email;
    private final String password;
    private final int failedLoginAttempts;
    private final LocalDateTime lockedUntil;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    // Constructor for creating a new user
    public User(String name, String email, String hashedPassword) {
        this.id = UUID.randomUUID();
        this.name = name;
        this.email = email;
        this.failedLoginAttempts = 0;
        this.lockedUntil = null;
        this.password = hashedPassword;
    }

    // Constructor for reconstructing from DB
    public User(UUID id, String name, String email, String password,
                int failedLoginAttempts, LocalDateTime lockedUntil,
                LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.password = password;
        this.failedLoginAttempts = failedLoginAttempts;
        this.lockedUntil = lockedUntil;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public User withUpdatedDetails(String name, String email) {
        return new User(id, name, email, password,
                failedLoginAttempts, lockedUntil, createdAt, LocalDateTime.now());
    }

    public User withPassword(String hashedPassword) {
        return new User(this.id, this.name, this.email, hashedPassword,
                this.failedLoginAttempts, this.lockedUntil, this.createdAt, LocalDateTime.now());
    }

    public User incrementFailedAttempts() {
        return new User(id, name, email, password,
                failedLoginAttempts + 1, lockedUntil, createdAt, updatedAt);
    }

    public User resetFailedAttempts() {
        return new User(id, name, email, password,
                0, null, createdAt, updatedAt);
    }

    public User lockUntil(LocalDateTime time) {
        return new User(id, name, email, password,
                failedLoginAttempts, time, createdAt, updatedAt);
    }

    public UUID getId() { return id; }
    public String getName() { return name; }
    public String getEmail() { return email; }
    public String getPassword() { return password; }
    public int getFailedLoginAttempts() { return failedLoginAttempts; }
    public LocalDateTime getLockedUntil() { return lockedUntil; }
    public LocalDateTime getCreatedAt() { return createdAt; }
    public LocalDateTime getUpdatedAt() { return updatedAt; }
}