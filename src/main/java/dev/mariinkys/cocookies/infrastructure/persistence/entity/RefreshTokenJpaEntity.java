package dev.mariinkys.cocookies.infrastructure.persistence.entity;

import jakarta.persistence.*;
import java.time.LocalDateTime;

@Entity
@Table(name = "refresh_tokens")
public class RefreshTokenJpaEntity {

    @Id
    @Column(columnDefinition = "TEXT", updatable = false, nullable = false)
    private String id;

    @Column(nullable = false, unique = true, length = 512)
    private String token;

    @Column(name = "user_email", nullable = false, length = 255)
    private String userEmail;

    @Column(name = "expires_at", columnDefinition = "TEXT", nullable = false)
    private LocalDateTime expiresAt;

    @Column(name = "revoked", columnDefinition = "INTEGER", nullable = false)
    private boolean revoked;

    @Column(name = "created_at", columnDefinition = "TEXT", nullable = false, updatable = false)
    private LocalDateTime createdAt;

    @PrePersist
    protected void onCreate() {
        createdAt = LocalDateTime.now();
    }

    protected RefreshTokenJpaEntity() {}

    public RefreshTokenJpaEntity(String id, String token, String userEmail,
                                 LocalDateTime expiresAt, boolean revoked,
                                 LocalDateTime createdAt) {
        this.id = id;
        this.token = token;
        this.userEmail = userEmail;
        this.expiresAt = expiresAt;
        this.revoked = revoked;
        this.createdAt = createdAt;
    }

    public String getId() { return id; }
    public String getToken() { return token; }
    public String getUserEmail() { return userEmail; }
    public LocalDateTime getExpiresAt() { return expiresAt; }
    public boolean isRevoked() { return revoked; }
    public LocalDateTime getCreatedAt() { return createdAt; }
}