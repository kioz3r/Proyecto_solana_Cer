# 📺 Suscripciones — Programa en Solana con Anchor

Un programa de gestión de suscripciones desplegado en la blockchain de Solana, desarrollado con el framework **Anchor**. Permite crear plataformas con sus respectivos suscriptores, administrarlos y filtrarlos de forma descentralizada.

---

## 🚀 Características

- ✅ Crear una plataforma de suscripciones vinculada a un owner
- ✅ Agregar suscriptores con nombre, tipo de suscripción y meses contratados
- ✅ Listar todos los suscriptores de una plataforma
- ✅ Buscar suscriptores por tipo de suscripción (ej. `"Premium"`, `"Basica"`, `"VIP"`)
- ✅ Activar / desactivar suscripciones de un suscriptor
- ✅ Eliminar suscriptores de la plataforma

---

---

## 📋 Instrucciones del Programa

### `crear_plataforma`
Inicializa una nueva cuenta `Plataforma` asociada al owner mediante un PDA.

| Parámetro | Tipo | Descripción |
|---|---|---|
| `nombre` | `String` | Nombre de la plataforma (máx. 60 caracteres) |

---

### `agregar_suscriptor`
Agrega un nuevo suscriptor a la plataforma. Solo el owner puede ejecutar esta instrucción.

| Parámetro | Tipo | Descripción |
|---|---|---|
| `nombre` | `String` | Nombre del suscriptor (máx. 60 caracteres) |
| `tipo_suscripcion` | `String` | Tipo de suscripción (ej. `"Premium"`) |
| `meses_contratados` | `u16` | Duración de la suscripción en meses |

---

### `ver_suscriptores`
Imprime en los logs del programa la lista completa de suscriptores. Solo el owner puede ejecutar esta instrucción.

---

### `buscar_por_tipo`
Filtra y muestra en logs los suscriptores que coincidan con un tipo de suscripción específico.

| Parámetro | Tipo | Descripción |
|---|---|---|
| `tipo` | `String` | Tipo de suscripción a buscar |

---

### `alternar_estado_suscripcion`
Cambia el estado `activa` de un suscriptor de `true` a `false` o viceversa. Solo el owner puede ejecutar esta instrucción.

| Parámetro | Tipo | Descripción |
|---|---|---|
| `nombre` | `String` | Nombre del suscriptor a modificar |

---

### `eliminar_suscriptor`
Elimina un suscriptor de la lista. Solo el owner puede ejecutar esta instrucción.

| Parámetro | Tipo | Descripción |
|---|---|---|
| `nombre` | `String` | Nombre del suscriptor a eliminar |

---

## 🏗️ Estructura de Cuentas

### `Plataforma`
Cuenta principal del programa, almacenada en la blockchain.

```rust
pub struct Plataforma {
    owner: Pubkey,             // Propietario de la plataforma
    nombre: String,            // Nombre de la plataforma
    suscriptores: Vec<Suscriptor>, // Lista de suscriptores (máx. 50)
}
```

### `Suscriptor`
Estructura embebida dentro de `Plataforma`.

```rust
pub struct Suscriptor {
    nombre: String,           // Nombre del suscriptor
    tipo_suscripcion: String, // Tipo de suscripción
    meses_contratados: u16,   // Duración en meses
    activa: bool,             // Estado de la suscripción
}
```

---

## ⚠️ Errores Personalizados

| Código | Nombre | Mensaje |
|---|---|---|
| `NoEresElOwner` | Acceso denegado | "Error, no eres el propietario de la plataforma" |
| `SuscriptorNoExiste` | No encontrado | "Error, el suscriptor no existe" |

---

## 🔐 Seguridad

- Todas las instrucciones sensibles validan que el firmante sea el **owner** de la plataforma.
- La cuenta `Plataforma` se genera como un **PDA** derivado de la semilla `"plataforma"` y la clave pública del owner, garantizando unicidad por wallet.

---

## 📄 Licencia

MIT License — libre para usar, modificar y distribuir.




