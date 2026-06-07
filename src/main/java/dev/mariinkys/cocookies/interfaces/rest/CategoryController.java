package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.CategoryUseCase;
import dev.mariinkys.cocookies.application.port.UserUseCase;
import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.interfaces.dto.PageResponse;
import dev.mariinkys.cocookies.interfaces.dto.category.CategoryRequest;
import dev.mariinkys.cocookies.interfaces.dto.category.CategoryResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/categories")
public class CategoryController {

    private final CategoryUseCase categoryUseCase;
    private final UserUseCase userUseCase;

    public CategoryController(CategoryUseCase categoryUseCase,  UserUseCase userUseCase) {
        this.categoryUseCase = categoryUseCase;
        this.userUseCase = userUseCase;
    }

    @GetMapping
    public ResponseEntity<PageResponse<CategoryResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "name") String sortBy,
            @RequestParam(defaultValue = "asc") String sortDir,
            @RequestParam(defaultValue = "") String search,
            @AuthenticationPrincipal UserDetails requester) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = categoryUseCase.getAllCategories(search, pageable, requesterContext(requester))
                .map(CategoryResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    @GetMapping("selector")
    public ResponseEntity<List<CategoryResponse>> getSelector(@AuthenticationPrincipal UserDetails requester) {
        var result = categoryUseCase.getSelector(requesterContext(requester)).stream()
                .map(CategoryResponse::from).toList();

        return ResponseEntity.ok(result);
    }

    @GetMapping("/{id}")
    public ResponseEntity<CategoryResponse> getById(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        return ResponseEntity.ok(CategoryResponse.from(categoryUseCase.getCategoryById(id,  requesterContext(requester))));
    }

    @PostMapping
    public ResponseEntity<CategoryResponse> create(
            @Valid @RequestBody CategoryRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var category = categoryUseCase.createCategory(
                request.name(), request.description(), requesterContext(requester)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(CategoryResponse.from(category));
    }

    @PutMapping("/{id}")
    public ResponseEntity<CategoryResponse> update(
            @PathVariable UUID id,
            @Valid @RequestBody CategoryRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var category = categoryUseCase.updateCategory(id, request.name(), request.description(), requesterContext(requester));
        return ResponseEntity.ok(CategoryResponse.from(category));
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        categoryUseCase.deleteCategory(id, requesterContext(requester));
        return ResponseEntity.noContent().build();
    }

    private RequesterContext requesterContext(UserDetails userDetails) {
        var user = userUseCase.getUserByEmail(userDetails.getUsername());
        return new RequesterContext(user.getId(), userDetails.getUsername());
    }
}