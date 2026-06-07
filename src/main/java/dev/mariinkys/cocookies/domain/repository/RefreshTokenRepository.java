package dev.mariinkys.cocookies.domain.repository;

import dev.mariinkys.cocookies.domain.models.RefreshToken;
import java.util.Optional;

public interface RefreshTokenRepository {
    RefreshToken save(RefreshToken token);
    Optional<RefreshToken> findByToken(String token);
    void revokeAllByUserEmail(String email); // used on logout
    void deleteExpired(); // for scheduled cleanup
}