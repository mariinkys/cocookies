// en.ts
export default {
  common: {
    actions: {
      cancel: 'Cancel',
      close: 'Close',
      apply: 'Apply',
      saveChanges: 'Save Changes',
      refresh: 'Refresh',
      back: 'Go Back',
      create: 'Create',
      delete: 'Delete',
      export: 'Export',
      send: 'Send',
      preview: 'Preview',
      addFiles: 'Add Files',
      signOut: 'Sign out',
      save: 'Save',
      remove: 'Remove',
      clear: 'Clear',
    },

    feedback: {
      error: 'Error',
      saved: 'Saved',
      created: 'Created',
      deleted: 'Deleted',
      notAvailable: 'Not Available',
    },

    fields: {
      name: 'Name',
      surname: 'Surname',
      secondSurname: 'Second Surname',
      email: 'Email',
      password: 'Password',
      currentPassword: 'Current Password',
      newPassword: 'New Password',
      confirmPassword: 'Confirm Password',
      notes: 'Notes',
      description: 'Description',
      optional: '(optional)',
      createdAt: 'Created At',
      updatedAt: 'Updated At',
    },

    placeholders: {
      enterName: 'Enter name',
      enterSurname: 'Enter surname',
      enterSecondSurname: 'Enter second surname',
      enterEmail: 'Enter email',
      enterYourEmail: 'Enter your email',
      enterYourName: 'Enter your name',
      enterPassword: 'Enter password',
      enterCurrentPassword: 'Enter current password',
      enterNewPassword: 'Enter new password',
      repeatNewPassword: 'Repeat new password',
      confirmPassword: 'Repeat your password',
      additionalNotes: 'Any additional notes',
      search: 'Search...',
      searchRecipes: 'Search recipes…',
    },

    validation: {
      required: 'This field is required',
      emailRequired: 'Email is required',
      passwordRequired: 'Password is required',
      invalidEmail: 'Must be a valid email',
      emailInvalid: 'Email must be valid',
      nameRequired: 'Name is required',
      surnameRequired: 'Surname is required',
      currentPasswordRequired: 'Current password is required',
      newPasswordRequired: 'New password is required',
      confirmNewPasswordRequired: 'Please confirm the new password',
      confirmPasswordRequired: 'Please confirm your password',
      interestsRequired: 'Select at least one interest',
      passwordsMismatch: 'Passwords do not match',
      passwordsDoNotMatch: 'Passwords do not match',
      passwordMinLength: 'Password must be at least 8 characters',
      max100: 'Must not exceed 100 characters',
      min8: 'Must be at least 8 characters',
      min1: 'Must be at least 1',
      exceedsChars: 'Exceeds the maximum number of characters',
      exceedsMax: 'Exceeds maximum',
      intOnly: 'Integer numbers only',
    },

    states: {
      emptyValue: '—',
      noResults: 'No results found',
    },

    theme: {
      switchToLightMode: 'Switch to light mode',
      switchToDarkMode: 'Switch to dark mode',
      toggleMenu: 'Toggle Menu',
    },

    sort: {
      titleAsc: 'Title (A → Z)',
      titleDesc: 'Title (Z → A)',
      newestFirst: 'Newest first',
      oldestFirst: 'Oldest first',
    },
  },

  navigation: {
    items: {
      sharedRecipes: 'Shared Recipes',
      newAccount: "Don't have an account? Create one",
      alreadyHaveAccount: 'Already have an account? Sign in',
    },
  },

  auth: {
    login: {
      title: 'Sign in',
      description: 'Welcome back — please enter your details',
      submit: 'Sign in',
      noAccount: "Don't have an account?",
      errors: {
        loginFailed: 'Login failed',
      },
    },
    register: {
      title: 'Create an account',
      description: 'Fill in the details below to get started',
      submit: 'Create account',
      success: 'Account created successfully',
      errors: {
        registerFailed: 'Registration failed, please try again',
      },
    },
  },

  users: {
    titles: {
      edit: 'Edit User',
    },

    descriptions: {
      edit: "Update the user's details below",
    },

    actions: {
      delete: 'Delete user',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterName',
        required: '@:common.validation.nameRequired',
        max: 'Name must not exceed 100 characters',
      },
      email: {
        label: '@:common.fields.email',
        placeholder: '@:common.placeholders.enterEmail',
        required: '@:common.validation.emailRequired',
        invalid: '@:common.validation.emailInvalid',
      },
      password: {
        label: '@:common.fields.password',
        placeholder: '@:common.placeholders.enterPassword',
        required: '@:common.validation.passwordRequired',
        min: 'Password must be at least 8 characters',
      },
    },

    passwordCard: {
      title: 'Change Password',
      submit: 'Update Password',
      fields: {
        currentPassword: {
          label: '@:common.fields.currentPassword',
          placeholder: '@:common.placeholders.enterCurrentPassword',
          required: '@:common.validation.currentPasswordRequired',
        },
        newPassword: {
          label: '@:common.fields.newPassword',
          placeholder: '@:common.placeholders.enterNewPassword',
          required: '@:common.validation.newPasswordRequired',
          min: 'Password must be at least 8 characters',
        },
        confirmPassword: {
          label: '@:common.fields.confirmPassword',
          placeholder: '@:common.placeholders.repeatNewPassword',
          required: '@:common.validation.confirmNewPasswordRequired',
          mismatch: '@:common.validation.passwordsMismatch',
        },
      },
      successTitle: 'Password updated',
      successDetail: 'Password changed successfully.',
      error: 'Failed to update password. Check your current password and try again.',
    },

    messages: {
      updated: 'User updated successfully',
      updateError: 'Failed to update user',
      loadError: 'Failed to load user',
    },
  },

  recipes: {
    titles: {
      list: 'My Recipes',
      create: 'New Recipe',
      edit: 'Edit Recipe',
      shared: 'Shared Recipes',
    },

    descriptions: {
      edit: 'Update the recipe details below',
      create: 'Create a new recipe',
    },

    list: {
      count: 'No recipes | 1 recipe | {count} recipes',
      empty: 'No recipes yet',
    },

    badges: {
      shared: 'Shared',
    },

    actions: {
      createNew: 'New Recipe',
      createFirst: 'Create your first recipe',
      edit: 'Edit recipe',
      delete: 'Delete recipe',
      create: 'Create Recipe',
      addStep: 'Add Step',
      editStep: 'Edit Step',
      saveAndAddAnother: 'Save & Keep Adding',
      saveAndClose: 'Save & Close',
      removeStep: 'Remove step',
      moveStepUp: 'Move step up',
      moveStepDown: 'Move step down',
      addIngredient: 'Add Ingredient',
      editIngredient: 'Edit ingredient',
      removeIngredient: 'Remove ingredient',
      downloadPdf: 'Download PDF',
      pdfError: 'Failed to generate PDF',
      notAvailable: 'Not yet available',
    },

    sections: {
      basicInfo: 'Basic Information',
      steps: 'Steps',
      ingredients: 'Ingredients',
      organisation: 'Organisation',
      timing: 'Timing & Servings',
      visibility: 'Visibility',
      atAGlance: 'At a Glance',
      metadata: 'Details',
    },

    fields: {
      minutes: 'min',
      minutesShort: 'min',
      servingsList: 'No servings | 1 serving | {count} servings',
      totalTime: 'Total Time',
      title: {
        label: 'Title',
        placeholder: 'e.g. Spaghetti Carbonara',
        required: 'Title is required',
        max: 'Title must not exceed 255 characters',
      },
      description: { label: 'Description', placeholder: 'Briefly describe the recipe…' },
      imageUrl: { label: 'Image URL', placeholder: 'https://…' },
      category: { label: 'Category', placeholder: 'Select a category' },
      difficulty: { label: 'Difficulty', placeholder: 'Select a difficulty' },
      prepTime: { label: 'Prep Time', placeholder: '15' },
      cookTime: { label: 'Cook Time', placeholder: '30' },
      servings: { label: 'Servings', placeholder: '4', min: 'Servings must be at least 1' },
      shared: {
        label: 'Share this recipe publicly',
        help: 'Shared recipes are visible to all users.',
      },
      steps: {
        instructions: 'Instructions',
        instructionsPlaceholder: 'Describe this step…',
        instructionsRequired: 'Instructions are required',
        duration: 'Duration',
        durationPlaceholder: '10',
        required: 'At least one step is required',
        empty: 'No steps yet. Add your first step.',
      },
      ingredients: {
        name: 'Name',
        namePlaceholder: 'e.g. Flour',
        nameRequired: 'Ingredient name is required',
        quantity: 'Quantity',
        quantityPlaceholder: '200',
        unit: 'Unit',
        unitPlaceholder: 'e.g. g, ml, cup',
        notes: 'Notes',
        notesPlaceholder: 'e.g. sifted',
        required: 'At least one ingredient is required',
        empty: 'No ingredients yet. Add your first ingredient.',
      },
      nutrition: {
        title: 'Nutrition',
        add: 'Add nutrition',
        remove: 'Remove nutrition',
        empty: 'No nutrition info added yet.',
        servingSizeValue: 'Serving size',
        servingSizeUnit: 'Unit',
        calories: 'Calories (kcal)',
        proteinG: 'Protein (g)',
        carbsG: 'Carbohydrates (g)',
        sugarG: 'Sugars (g)',
        fatG: 'Fat (g)',
        saturatedFatG: 'Saturated fat (g)',
        fiberG: 'Fiber (g)',
        sodiumMg: 'Sodium (mg)',
        perServing: 'Per {value}{unit}',
        total: 'Whole recipe',
        totalShort: 'total',
        forServings: 'for {count} serving(s)',
        footnote: 'Values per {value}{unit} · {servings} serving(s) total',
      },
    },

    messages: {
      loadListError: 'Failed to load recipes',
      loadError: 'Failed to load recipe',
      updated: 'Recipe updated successfully',
      updateError: 'Failed to update recipe',
      created: 'Recipe created successfully',
      createError: 'Failed to create recipe',
      notAvailable: 'Feature not yet available',
    },

    deleteDialog: {
      title: 'Delete Recipe',
      message: 'Are you sure you want to delete "{name}"? This action cannot be undone.',
      messageGeneric: 'Are you sure you want to delete this recipe? This action cannot be undone.',
      success: '"{name}" was deleted successfully.',
      error: 'Failed to delete recipe. Please try again.',
      deletedSuccess: 'Recipe deleted successfully.',
    },

    import: {
      trigger: 'Import from AI',
      title: 'Import Recipe',
      modes: {
        file: 'Image / PDF',
        url: 'URL',
      },
      fileLabel: 'Upload file',
      filePlaceholder: 'Click to choose a file',
      fileHint: 'Supports JPG, PNG, WebP, PDF',
      urlLabel: 'Recipe URL',
      urlPlaceholder: 'https://...',
      submit: 'Import',
      prompt:
        'Extract the recipe from this content. Return title, description, ingredients with quantities and units, and numbered steps with durations where available.',
      disclaimer: 'Review all fields after importing — AI results may need corrections.',
      error: 'Could not extract a recipe. Try a different file or URL.',
    },
  },

  categories: {
    actions: { create: 'New category' },
    dialog: {
      title: 'New Category',
      namePlaceholder: 'e.g. Desserts',
      descriptionPlaceholder: 'Briefly describe this category…',
    },
    messages: {
      created: 'Category created successfully',
      createError: 'Failed to create category',
    },
  },

  difficulties: {
    actions: { create: 'New difficulty' },
    dialog: {
      title: 'New Difficulty',
      namePlaceholder: 'e.g. Hard',
      descriptionPlaceholder: 'Briefly describe this difficulty…',
    },
    messages: {
      created: 'Difficulty created successfully',
      createError: 'Failed to create difficulty',
    },
  },
}
