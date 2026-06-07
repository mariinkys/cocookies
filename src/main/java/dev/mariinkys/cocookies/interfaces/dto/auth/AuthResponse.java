package dev.mariinkys.cocookies.interfaces.dto.auth;

import dev.mariinkys.cocookies.domain.models.User;
import java.util.UUID;

public record AuthResponse(UUID id, String email) {
    public static AuthResponse from(User user) {
        return new AuthResponse(user.getId(), user.getEmail());
    }
}
