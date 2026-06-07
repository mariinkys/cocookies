package dev.mariinkys.cocookies.infrastructure.security.filter;

import io.github.bucket4j.Bandwidth;
import io.github.bucket4j.Bucket;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.jspecify.annotations.NullMarked;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.core.annotation.Order;
import org.springframework.http.MediaType;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;

import java.io.IOException;
import java.time.Duration;
import java.time.LocalDateTime;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

@Component
@NullMarked
@Order(1) // runs before everything else, including Spring Security
public class RateLimitFilter extends OncePerRequestFilter {

    @Value("${app.security.trust-proxy:false}")
    private boolean trustProxy;

    private static final List<String> RATE_LIMITED = List.of(
            "/api/auth/login", "/api/auth/register"
    );

    // Per-IP bucket cache, entries naturally expire when IPs stop making requests
    private final Map<String, Bucket> buckets = new ConcurrentHashMap<>();

    @Override
    protected void doFilterInternal(HttpServletRequest request,
                                    HttpServletResponse response,
                                    FilterChain chain) throws ServletException, IOException {

        // OPTIONS preflight requests must never consume rate limit tokens
        if ("OPTIONS".equalsIgnoreCase(request.getMethod())) {
            chain.doFilter(request, response);
            return;
        }

        if (RATE_LIMITED.stream().noneMatch(request.getRequestURI()::equals)) {
            chain.doFilter(request, response);
            return;
        }

        String ip = extractIp(request);
        Bucket bucket = buckets.computeIfAbsent(ip, _ -> newBucket());

        if (bucket.tryConsume(1)) {
            chain.doFilter(request, response);
        } else {
            response.setStatus(429);
            response.setContentType(MediaType.APPLICATION_JSON_VALUE);
            response.getWriter().write("""
            {"status":429,"message":"Too many requests, please try again later.","timestamp":"%s"}
            """.formatted(LocalDateTime.now()));
        }
    }

    private Bucket newBucket() {
        return Bucket.builder()
                .addLimit(Bandwidth.builder()
                        .capacity(5)
                        .refillGreedy(5, Duration.ofMinutes(1))
                        .build())
                .build();
    }

    private String extractIp(HttpServletRequest request) {
        if (trustProxy) {
            String forwarded = request.getHeader("X-Forwarded-For");
            if (forwarded != null && !forwarded.isBlank()) {
                return forwarded.split(",")[0].trim();
            }
        }
        return request.getRemoteAddr();
    }
}