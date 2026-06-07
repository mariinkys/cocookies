package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.UserUseCase;
import dev.mariinkys.cocookies.application.service.RefreshTokenService;
import dev.mariinkys.cocookies.domain.models.User;
import dev.mariinkys.cocookies.infrastructure.security.cookie.CookieService;
import dev.mariinkys.cocookies.infrastructure.security.jwt.JwtService;
import dev.mariinkys.cocookies.interfaces.dto.auth.AuthRequest;
import dev.mariinkys.cocookies.interfaces.dto.auth.AuthResponse;
import dev.mariinkys.cocookies.interfaces.dto.auth.RegisterRequest;
import dev.mariinkys.cocookies.interfaces.dto.user.UserResponse;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import jakarta.validation.Valid;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final AuthenticationManager authManager;
    private final JwtService jwtService;
    private final RefreshTokenService refreshTokenService;
    private final UserUseCase userUseCase;
    private final CookieService cookieService;

    public AuthController(AuthenticationManager authManager,
                          JwtService jwtService,
                          RefreshTokenService refreshTokenService,
                          UserUseCase userUseCase,
                          CookieService cookieService) {
        this.authManager = authManager;
        this.jwtService = jwtService;
        this.refreshTokenService = refreshTokenService;
        this.userUseCase = userUseCase;
        this.cookieService = cookieService;
    }

    // TODO: Maybe this makes more sense in the user controller?
    @PostMapping("/register")
    public ResponseEntity<UserResponse> register(@Valid @RequestBody RegisterRequest request) {
        userUseCase.createUser(request.name(), request.email(), request.password());
        var user = userUseCase.getUserByEmail(request.email());
        return ResponseEntity.status(HttpStatus.CREATED).body(UserResponse.from(user));
    }

    @PostMapping("/login")
    public ResponseEntity<AuthResponse> login(@Valid @RequestBody AuthRequest request,
                                              HttpServletResponse response) {
        return ResponseEntity.ok(issueTokensAndBuildResponse(request.email(), request.password(), response));
    }

    @PostMapping("/refresh")
    public ResponseEntity<AuthResponse> refresh(HttpServletRequest request,
                                                HttpServletResponse response) {
        String refreshToken = extractRefreshTokenFromCookie(request);
        if (refreshToken == null) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).build();
        }

        String email = refreshTokenService.validateAndRotate(refreshToken);
        User user = userUseCase.getUserByEmail(email);

        setTokenCookies(email, response);

        return ResponseEntity.ok(AuthResponse.from(user));
    }

    @PostMapping("/logout")
    public ResponseEntity<Void> logout(@AuthenticationPrincipal UserDetails userDetails,
                                       HttpServletResponse response) {
        refreshTokenService.revokeAllForUser(userDetails.getUsername());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.clearAccessTokenCookie().toString());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.clearRefreshTokenCookie().toString());
        return ResponseEntity.noContent().build();
    }

    // Authenticates, sets cookies, returns user info shared by login and register
    private AuthResponse issueTokensAndBuildResponse(String email, String rawPassword,
                                                     HttpServletResponse response) {

        authManager.authenticate(new UsernamePasswordAuthenticationToken(email, rawPassword));

        User user = userUseCase.getUserByEmail(email);
        setTokenCookies(email, response);
        return AuthResponse.from(user);
    }

    private void setTokenCookies(String email, HttpServletResponse response) {
        String accessToken = jwtService.generateToken(email);
        String refreshToken = refreshTokenService.createRefreshToken(email);
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.createAccessTokenCookie(accessToken).toString());
        response.addHeader(HttpHeaders.SET_COOKIE, cookieService.createRefreshTokenCookie(refreshToken).toString());
    }

    private String extractRefreshTokenFromCookie(HttpServletRequest request) {
        if (request.getCookies() == null) return null;
        for (var cookie : request.getCookies()) {
            if ("refresh_token".equals(cookie.getName())) return cookie.getValue();
        }
        return null;
    }
}