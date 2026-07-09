// es.ts
export default {
  common: {
    actions: {
      cancel: 'Cancelar',
      close: 'Cerrar',
      apply: 'Aplicar',
      saveChanges: 'Guardar cambios',
      refresh: 'Actualizar',
      back: 'Volver',
      create: 'Crear',
      delete: 'Eliminar',
      export: 'Exportar',
      send: 'Enviar',
      preview: 'Vista previa',
      addFiles: 'Añadir archivos',
      signOut: 'Cerrar sesión',
      save: 'Guardar',
      remove: 'Eliminar',
    },

    feedback: {
      error: 'Error',
      saved: 'Guardado',
      created: 'Creado',
      deleted: 'Eliminado',
      notAvailable: 'No Disponible',
    },

    fields: {
      name: 'Nombre',
      surname: 'Apellido',
      secondSurname: 'Segundo apellido',
      email: 'Correo electrónico',
      password: 'Contraseña',
      currentPassword: 'Contraseña actual',
      newPassword: 'Nueva contraseña',
      confirmPassword: 'Confirmar contraseña',
      notes: 'Notas',
      description: 'Descripción',
      optional: '(opcional)',
      createdAt: 'Creado el',
      updatedAt: 'Actualizado el',
    },

    placeholders: {
      enterName: 'Introduce el nombre',
      enterSurname: 'Introduce el apellido',
      enterSecondSurname: 'Introduce el segundo apellido',
      enterEmail: 'Introduce el correo',
      enterYourEmail: 'Introduce tu correo electrónico',
      enterYourName: 'Introduce tu nombre',
      enterPassword: 'Introduce la contraseña',
      enterCurrentPassword: 'Introduce la contraseña actual',
      enterNewPassword: 'Introduce la nueva contraseña',
      repeatNewPassword: 'Repite la nueva contraseña',
      confirmPassword: 'Repite tu contraseña',
      additionalNotes: 'Notas adicionales',
      search: 'Buscar...',
      searchRecipes: 'Buscar recetas…',
    },

    validation: {
      required: 'Este campo es obligatorio',
      emailRequired: 'El correo electrónico es obligatorio',
      passwordRequired: 'La contraseña es obligatoria',
      invalidEmail: 'Debe ser un correo electrónico válido',
      emailInvalid: 'El correo electrónico debe ser válido',
      nameRequired: 'El nombre es obligatorio',
      surnameRequired: 'El apellido es obligatorio',
      currentPasswordRequired: 'La contraseña actual es obligatoria',
      newPasswordRequired: 'La nueva contraseña es obligatoria',
      confirmNewPasswordRequired: 'Por favor confirma la nueva contraseña',
      confirmPasswordRequired: 'Por favor confirma tu contraseña',
      interestsRequired: 'Selecciona al menos un interés',
      passwordsMismatch: 'Las contraseñas no coinciden',
      passwordsDoNotMatch: 'Las contraseñas no coinciden',
      passwordMinLength: 'La contraseña debe tener al menos 8 caracteres',
      max100: 'No debe superar los 100 caracteres',
      min8: 'Debe tener al menos 8 caracteres',
      min1: 'Debe ser al menos 1',
      exceedsChars: 'Excede el máximo de caracteres',
      exceedsMax: 'Excede el máximo',
      intOnly: 'Solo números enteros',
    },

    states: {
      emptyValue: '—',
      noResults: 'Sin resultados',
    },

    theme: {
      switchToLightMode: 'Cambiar a modo claro',
      switchToDarkMode: 'Cambiar a modo oscuro',
      toggleMenu: 'Abrir menú',
    },

    sort: {
      titleAsc: 'Título (A → Z)',
      titleDesc: 'Título (Z → A)',
      newestFirst: 'Más recientes primero',
      oldestFirst: 'Más antiguos primero',
    },
  },

  navigation: {
    items: {
      sharedRecipes: 'Recetas compartidas',
      newAccount: '¿No tienes cuenta? Crea una',
      alreadyHaveAccount: '¿Ya tienes cuenta? Inicia sesión',
    },
  },

  auth: {
    login: {
      title: 'Iniciar sesión',
      description: 'Bienvenido de nuevo — introduce tus datos',
      submit: 'Iniciar sesión',
      noAccount: '¿No tienes cuenta?',
      errors: {
        loginFailed: 'Error al iniciar sesión',
      },
    },
    register: {
      title: 'Crear una cuenta',
      description: 'Rellena los datos para empezar',
      submit: 'Crear cuenta',
      success: 'Cuenta creada correctamente',
      errors: {
        registerFailed: 'Error al registrarse, inténtalo de nuevo',
      },
    },
  },

  users: {
    titles: {
      edit: 'Editar usuario',
    },

    descriptions: {
      edit: 'Actualiza los datos del usuario',
    },

    actions: {
      delete: 'Eliminar usuario',
    },

    fields: {
      name: {
        label: '@:common.fields.name',
        placeholder: '@:common.placeholders.enterName',
        required: '@:common.validation.nameRequired',
        max: 'El nombre no debe superar los 100 caracteres',
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
        min: 'La contraseña debe tener al menos 8 caracteres',
      },
    },

    passwordCard: {
      title: 'Cambiar contraseña',
      submit: 'Actualizar contraseña',
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
          min: 'La contraseña debe tener al menos 8 caracteres',
        },
        confirmPassword: {
          label: '@:common.fields.confirmPassword',
          placeholder: '@:common.placeholders.repeatNewPassword',
          required: '@:common.validation.confirmNewPasswordRequired',
          mismatch: '@:common.validation.passwordsMismatch',
        },
      },
      successTitle: 'Contraseña actualizada',
      successDetail: 'La contraseña se ha cambiado correctamente.',
      error:
        'Error al actualizar la contraseña. Comprueba tu contraseña actual e inténtalo de nuevo.',
    },

    messages: {
      updated: 'Usuario actualizado correctamente',
      updateError: 'Error al actualizar el usuario',
      loadError: 'Error al cargar el usuario',
    },
  },

  recipes: {
    titles: {
      list: 'Mis recetas',
      create: 'Nueva receta',
      edit: 'Editar receta',
      shared: 'Recetas compartidas',
    },

    descriptions: {
      edit: 'Actualiza los detalles de la receta',
      create: 'Crea una nueva receta',
    },

    list: {
      count: 'Sin recetas | 1 receta | {count} recetas',
      empty: 'Todavía no hay recetas',
    },

    badges: {
      shared: 'Compartida',
    },

    actions: {
      createNew: 'Nueva receta',
      createFirst: 'Crea tu primera receta',
      edit: 'Editar receta',
      delete: 'Eliminar receta',
      create: 'Crear receta',
      addStep: 'Añadir paso',
      editStep: 'Editar paso',
      saveAndAddAnother: 'Guardar y Seguir',
      saveAndClose: 'Guardar y Cerrar',
      removeStep: 'Eliminar paso',
      moveStepUp: 'Subir paso',
      moveStepDown: 'Bajar paso',
      addIngredient: 'Añadir ingrediente',
      editIngredient: 'Editar ingrediente',
      removeIngredient: 'Eliminar ingrediente',
      downloadPdf: 'Descargar PDF',
      notAvailable: 'Función no disponible todavía',
    },

    sections: {
      basicInfo: 'Información básica',
      steps: 'Pasos',
      ingredients: 'Ingredientes',
      organisation: 'Organización',
      timing: 'Tiempos y raciones',
      visibility: 'Visibilidad',
      atAGlance: 'Resumen',
      metadata: 'Detalles',
    },

    fields: {
      minutes: 'min',
      minutesShort: 'min',
      servingsList: 'Sin raciones | 1 ración | {count} raciones',
      totalTime: 'Tiempo total',
      title: {
        label: 'Título',
        placeholder: 'p. ej. Espaguetis a la carbonara',
        required: 'El título es obligatorio',
        max: 'El título no debe superar los 255 caracteres',
      },
      description: { label: 'Descripción', placeholder: 'Describe brevemente la receta…' },
      imageUrl: { label: 'URL de imagen', placeholder: 'https://…' },
      category: { label: 'Categoría', placeholder: 'Selecciona una categoría' },
      difficulty: { label: 'Dificultad', placeholder: 'Selecciona una dificultad' },
      prepTime: { label: 'Tiempo de preparación', placeholder: '15' },
      cookTime: { label: 'Tiempo de cocción', placeholder: '30' },
      servings: { label: 'Raciones', placeholder: '4', min: 'Las raciones deben ser al menos 1' },
      shared: {
        label: 'Compartir esta receta públicamente',
        help: 'Las recetas compartidas son visibles para todos los usuarios.',
      },
      steps: {
        instructions: 'Instrucciones',
        instructionsPlaceholder: 'Describe este paso…',
        instructionsRequired: 'Las instrucciones son obligatorias',
        duration: 'Duración',
        durationPlaceholder: '10',
        required: 'Se necesita al menos un paso',
        empty: 'No hay pasos aún. Crea tu primer paso.',
      },
      ingredients: {
        name: 'Nombre',
        namePlaceholder: 'p. ej. Harina',
        nameRequired: 'El nombre del ingrediente es obligatorio',
        quantity: 'Cantidad',
        quantityPlaceholder: '200',
        unit: 'Unidad',
        unitPlaceholder: 'p. ej. g, ml, taza',
        notes: 'Notas',
        notesPlaceholder: 'p. ej. tamizada',
        required: 'Se necesita al menos un ingrediente',
        empty: 'No hay ingredientes aún. Añade el primero.',
      },
      nutrition: {
        title: 'Nutrición',
        add: 'Añadir nutrición',
        remove: 'Eliminar nutrición',
        empty: 'Aún no se ha añadido información nutricional.',
        servingSizeValue: 'Tamaño de ración',
        servingSizeUnit: 'Unidad',
        calories: 'Calorías (kcal)',
        proteinG: 'Proteínas (g)',
        carbsG: 'Carbohidratos (g)',
        sugarG: 'Azúcares (g)',
        fatG: 'Grasas (g)',
        saturatedFatG: 'Grasas saturadas (g)',
        fiberG: 'Fibra (g)',
        sodiumMg: 'Sodio (mg)',
        perServing: 'Por {value}{unit}',
        total: 'Receta completa',
        totalShort: 'total',
        forServings: 'para {count} ración(es)',
        footnote: 'Valores por {value}{unit} · {servings} ración(es) en total',
      },
    },

    messages: {
      loadListError: 'Error al cargar las recetas',
      loadError: 'Error al cargar la receta',
      updated: 'Receta actualizada correctamente',
      updateError: 'Error al actualizar la receta',
      created: 'Receta creada correctamente',
      createError: 'Error al crear la receta',
      pdfError: 'Error al generar el PDF',
    },

    deleteDialog: {
      title: 'Eliminar receta',
      message: '¿Seguro que quieres eliminar "{name}"? Esta acción no se puede deshacer.',
      messageGeneric: '¿Seguro que quieres eliminar esta receta? Esta acción no se puede deshacer.',
      success: '"{name}" se ha eliminado correctamente.',
      error: 'Error al eliminar la receta. Inténtalo de nuevo.',
      deletedSuccess: 'Receta eliminada correctamente.',
    },

    import: {
      trigger: 'Importar con IA',
      title: 'Importar receta',
      modes: {
        file: 'Imagen / PDF',
        url: 'URL',
      },
      fileLabel: 'Subir archivo',
      filePlaceholder: 'Haz clic para elegir un archivo',
      fileHint: 'Compatible con JPG, PNG, WebP, PDF',
      urlLabel: 'URL de la receta',
      urlPlaceholder: 'https://...',
      submit: 'Importar',
      prompt:
        'Extrae la receta de este contenido. Devuelve el título, la descripción, los ingredientes con cantidades y unidades, y los pasos numerados con duraciones si están disponibles.',
      disclaimer:
        'Revisa todos los campos tras importar — los resultados de la IA pueden necesitar correcciones.',
      error: 'No se ha podido extraer la receta. Prueba con otro archivo o URL.',
    },
  },

  categories: {
    actions: { create: 'Nueva categoría' },
    dialog: {
      title: 'Nueva categoría',
      namePlaceholder: 'p. ej. Postres',
      descriptionPlaceholder: 'Describe brevemente esta categoría…',
    },
    messages: {
      created: 'Categoría creada correctamente',
      createError: 'Error al crear la categoría',
    },
  },

  difficulties: {
    actions: { create: 'Nueva dificultad' },
    dialog: {
      title: 'Nueva dificultad',
      namePlaceholder: 'p. ej. Difícil',
      descriptionPlaceholder: 'Describe brevemente esta dificultad…',
    },
    messages: {
      created: 'Dificultad creada correctamente',
      createError: 'Error al crear la dificultad',
    },
  },
}
