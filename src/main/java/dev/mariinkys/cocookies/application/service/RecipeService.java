package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.application.exception.RecipeNotFoundException;
import dev.mariinkys.cocookies.application.port.PdfGenerator;
import dev.mariinkys.cocookies.application.port.RecipeUseCase;
import dev.mariinkys.cocookies.application.utils.RecipeTemplates;
import dev.mariinkys.cocookies.domain.models.*;
import dev.mariinkys.cocookies.domain.repository.CategoryRepository;
import dev.mariinkys.cocookies.domain.repository.DifficultyRepository;
import dev.mariinkys.cocookies.domain.repository.RecipeRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.util.*;

@Service
public class RecipeService implements RecipeUseCase {

    private final RecipeRepository recipeRepository;
    private final CategoryRepository categoryRepository;
    private final DifficultyRepository difficultyRepository;
    private final PdfGenerator pdfGenerator;

    public RecipeService(RecipeRepository recipeRepository,
                         CategoryRepository categoryRepository,
                         DifficultyRepository difficultyRepository,
                         PdfGenerator pdfGenerator) {
        this.recipeRepository = recipeRepository;
        this.categoryRepository = categoryRepository;
        this.difficultyRepository = difficultyRepository;
        this.pdfGenerator = pdfGenerator;
    }

    @Override
    @Transactional
    public Recipe createRecipe(String title, String description, UUID categoryId, UUID difficultyId,
                               Integer prepTime, Integer cookTime, int servings, String imageUrl,
                               boolean shared, List<RecipeStep> steps,
                               List<RecipeIngredient> ingredients, RecipeNutrition nutrition, RequesterContext requester) {
        Category category = resolveCategory(categoryId);
        Difficulty difficulty = resolveDifficulty(difficultyId);

        return recipeRepository.save(
                new Recipe(requester.id(), category, difficulty, title, description,
                        prepTime, cookTime, servings, imageUrl, shared, steps, ingredients, nutrition)
        );
    }

    @Override
    @Transactional(readOnly = true)
    public Recipe getRecipeById(UUID id, RequesterContext requester) {
        Recipe target =  recipeRepository.findById(id)
                .orElseThrow(() -> new RecipeNotFoundException(id));

        if (!target.isShared() && !target.getUserId().equals(requester.id())) {
            throw new AccessDeniedException("You can only access your own recipe");
        }

        return target;
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Recipe> getAllRecipes(String search, Pageable pageable, RequesterContext requester) {
        return recipeRepository.findAllByUserId(requester.id(), search, pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Recipe> getAllSharedRecipes(String search, Pageable pageable) {
        return recipeRepository.findAllShared(search, pageable);
    }

    @Override
    @Transactional
    public Recipe updateRecipe(UUID id, String title, String description, UUID categoryId,
                               UUID difficultyId, Integer prepTime, Integer cookTime,
                               int servings, String imageUrl, boolean shared,
                               List<RecipeStep> steps, List<RecipeIngredient> ingredients,
                               RecipeNutrition nutrition, RequesterContext requester) {
        Recipe existing = getRecipeById(id, requester);
        Category category = resolveCategory(categoryId);
        Difficulty difficulty = resolveDifficulty(difficultyId);

        return recipeRepository.save(
                existing.withUpdatedDetails(category, difficulty, title, description,
                        prepTime, cookTime, servings, imageUrl, shared, steps, ingredients, nutrition)
        );
    }

    @Override
    @Transactional
    public void deleteRecipe(UUID id, RequesterContext requester) {
        getRecipeById(id, requester);
        recipeRepository.deleteById(id);
    }

    @Override
    @Transactional
    public byte[] generateRecipePdf(UUID id, RecipeTemplates template, RequesterContext requester) {
        Recipe recipe = getRecipeById(id, requester);

        String templateName = switch (template) {
            case DEFAULT_ENGLISH -> "pdf/recipe-default-en";
            case DEFAULT_SPANISH -> "pdf/recipe-default-es";

            default -> "pdf/recipe-default-en";
        };

        Map<String, Object> vars = new HashMap<>();
        vars.put("recipe", buildPdfModel(recipe));
        vars.put("generatedAt", LocalDate.now().format(
                DateTimeFormatter.ofPattern("dd/MM/yyyy")));

        return pdfGenerator.generate(templateName, vars);
    }

    private Map<String, Object> buildPdfModel(Recipe recipe) {
        Map<String, Object> model = new HashMap<>();
        model.put("title",       recipe.getTitle());
        model.put("description", recipe.getDescription() != null ? recipe.getDescription() : "");
        model.put("category",    recipe.getCategory()   != null ? recipe.getCategory().getName()   : "");
        model.put("difficulty",  recipe.getDifficulty() != null ? recipe.getDifficulty().getName() : "");
        model.put("prepTime",    recipe.getPrepTime()   != null ? recipe.getPrepTime()   : 0);
        model.put("cookTime",    recipe.getCookTime()   != null ? recipe.getCookTime()   : 0);
        model.put("servings",    recipe.getServings());
        model.put("imageUrl",    recipe.getImageUrl()   != null ? recipe.getImageUrl()   : "");
        model.put("shared",      recipe.isShared());
        model.put("createdAt",   recipe.getCreatedAt() != null
                ? recipe.getCreatedAt().format(DateTimeFormatter.ofPattern("dd/MM/yyyy")) : "");
        model.put("updatedAt",   recipe.getCreatedAt() != null
                ? recipe.getUpdatedAt().format(DateTimeFormatter.ofPattern("dd/MM/yyyy")) : "");
        model.put("steps",       recipe.getSteps().stream()
                .sorted(Comparator.comparingInt(RecipeStep::getStepNumber))
                .toList());
        model.put("ingredients", recipe.getIngredients().stream()
                .sorted(Comparator.comparingInt(RecipeIngredient::getSortOrder))
                .toList());
        return model;
    }

    private Category resolveCategory(UUID id) {
        if (id == null) return null;
        return categoryRepository.findById(id)
                .orElseThrow(() -> new IllegalArgumentException("Category not found with id: " + id));
    }

    private Difficulty resolveDifficulty(UUID id) {
        if (id == null) return null;
        return difficultyRepository.findById(id)
                .orElseThrow(() -> new IllegalArgumentException("Difficulty not found with id: " + id));
    }
}