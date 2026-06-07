package dev.mariinkys.cocookies.domain.models;

import java.time.LocalDateTime;
import java.util.UUID;

public class RefreshToken {
    private final UUID id;
    private final String token;
    private final String userEmail;
    private final LocalDateTime expiresAt;
    private final boolean revoked;
    private LocalDateTime createdAt;

    // For creating a new refresh token
    public RefreshToken(String token, String userEmail, LocalDateTime expiresAt) {
        this.id = UUID.randomUUID();
        this.token = token;
        this.userEmail = userEmail;
        this.expiresAt = expiresAt;
        this.revoked = false;
    }

    // For reconstructing from DB
    public RefreshToken(UUID id, String token, String userEmail,
                        LocalDateTime expiresAt, boolean revoked, LocalDateTime createdAt) {
        this.id = id;
        this.token = token;
        this.userEmail = userEmail;
        this.expiresAt = expiresAt;
        this.revoked = revoked;
        this.createdAt = createdAt;
    }

    public boolean isExpired() {
        return LocalDateTime.now().isAfter(expiresAt);
    }

    public boolean isValid() {
        return !revoked && !isExpired();
    }

    // Returns a new instance
    public RefreshToken revoke() {
        return new RefreshToken(id, token, userEmail, expiresAt, true, createdAt);
    }

    public UUID getId() { return id; }
    public String getToken() { return token; }
    public String getUserEmail() { return userEmail; }
    public LocalDateTime getExpiresAt() { return expiresAt; }
    public boolean isRevoked() { return revoked; }
    public LocalDateTime getCreatedAt() { return createdAt; }
}