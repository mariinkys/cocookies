package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.RecipeUseCase;
import dev.mariinkys.cocookies.application.port.UserUseCase;
import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.application.utils.RecipeTemplates;
import dev.mariinkys.cocookies.domain.models.RecipeIngredient;
import dev.mariinkys.cocookies.domain.models.RecipeNutrition;
import dev.mariinkys.cocookies.domain.models.RecipeStep;
import dev.mariinkys.cocookies.interfaces.dto.PageResponse;
import dev.mariinkys.cocookies.interfaces.dto.recipe.RecipeRequest;
import dev.mariinkys.cocookies.interfaces.dto.recipe.RecipeResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/recipes")
public class RecipeController {

    private final RecipeUseCase recipeUseCase;
    private final UserUseCase userUseCase;

    public RecipeController(RecipeUseCase recipeUseCase, UserUseCase userUseCase) {
        this.recipeUseCase = recipeUseCase;
        this.userUseCase = userUseCase;
    }

    @GetMapping
    public ResponseEntity<PageResponse<RecipeResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "title") String sortBy,
            @RequestParam(defaultValue = "asc") String sortDir,
            @RequestParam(defaultValue = "") String search,
            @AuthenticationPrincipal UserDetails requester) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = recipeUseCase.getAllRecipes(search, pageable, requesterContext(requester))
                .map(RecipeResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    @GetMapping("/shared")
    public ResponseEntity<PageResponse<RecipeResponse>> getAllShared(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "title") String sortBy,
            @RequestParam(defaultValue = "asc") String sortDir,
            @RequestParam(defaultValue = "") String search) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = recipeUseCase.getAllSharedRecipes(search, pageable)
                .map(RecipeResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    @GetMapping("/{id}")
    public ResponseEntity<RecipeResponse> getById(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        return ResponseEntity.ok(RecipeResponse.from(recipeUseCase.getRecipeById(id, requesterContext(requester))));
    }

    @PostMapping
    public ResponseEntity<RecipeResponse> create(
            @Valid @RequestBody RecipeRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var recipe = recipeUseCase.createRecipe(
                request.title(), request.description(),
                request.categoryId(), request.difficultyId(),
                request.prepTime(), request.cookTime(),
                request.servings(), request.imageUrl(), request.shared(),
                toStepDomain(request), toIngredientDomain(request),
                toNutritionDomain(request),
                requesterContext(requester)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(RecipeResponse.from(recipe));
    }

    @PutMapping("/{id}")
    public ResponseEntity<RecipeResponse> update(
            @PathVariable UUID id,
            @Valid @RequestBody RecipeRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var recipe = recipeUseCase.updateRecipe(
                id, request.title(), request.description(),
                request.categoryId(), request.difficultyId(),
                request.prepTime(), request.cookTime(),
                request.servings(), request.imageUrl(), request.shared(),
                toStepDomain(request), toIngredientDomain(request),
                toNutritionDomain(request),
                requesterContext(requester)
        );
        return ResponseEntity.ok(RecipeResponse.from(recipe));
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        recipeUseCase.deleteRecipe(id, requesterContext(requester));
        return ResponseEntity.noContent().build();
    }

    @GetMapping("/{id}/pdf/{template}")
    public ResponseEntity<byte[]> generatePdf(@PathVariable UUID id, @PathVariable String template, @AuthenticationPrincipal UserDetails requester) {
        RecipeTemplates selectedTemplate = RecipeTemplates.fromString(template);
        byte[] pdf = recipeUseCase.generateRecipePdf(id, selectedTemplate,requesterContext(requester));

        return ResponseEntity.ok()
                .header(HttpHeaders.CONTENT_DISPOSITION,
                        "attachment; filename=\"recipe-" + id + ".pdf\"")
                .contentType(MediaType.APPLICATION_PDF)
                .body(pdf);
    }

    private List<RecipeStep> toStepDomain(RecipeRequest request) {
        return request.steps().stream()
                .map(s -> new RecipeStep(null, s.stepNumber(), s.instructions(), s.duration()))
                .toList();
    }

    private List<RecipeIngredient> toIngredientDomain(RecipeRequest request) {
        return request.ingredients().stream()
                .map(i -> new RecipeIngredient(null, i.name(), i.quantity(), i.unit(), i.notes(), i.sortOrder()))
                .toList();
    }

    private RecipeNutrition toNutritionDomain(RecipeRequest request) {
        if (request.nutrition() == null) return null;
        return new RecipeNutrition(
                request.nutrition().servingSizeValue(),
                request.nutrition().servingSizeUnit(),
                request.nutrition().calories(),
                request.nutrition().proteinG(),
                request.nutrition().carbsG(),
                request.nutrition().sugarG(),
                request.nutrition().fatG(),
                request.nutrition().saturatedFatG(),
                request.nutrition().fiberG(),
                request.nutrition().sodiumMg()
        );
    }

    private RequesterContext requesterContext(UserDetails userDetails) {
        var user = userUseCase.getUserByEmail(userDetails.getUsername());
        return new RequesterContext(user.getId(), userDetails.getUsername());
    }
}