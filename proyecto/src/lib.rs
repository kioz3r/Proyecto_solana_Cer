use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod suscripciones {
    use super::*;

    // region: Función para crear plataforma

     pub fn crear_plataforma(context: Context<NuevaPlataforma>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let suscriptores: Vec<Suscriptor> = Vec::new();

        context.accounts.plataforma.set_inner(Plataforma {
            owner: owner_id,
            nombre,
            suscriptores,
        });

        Ok(())
    }

    // endregion
    
    // region: Función para agregar un nuevo suscriptor

 pub fn agregar_suscriptor(
        context: Context<NuevoSuscriptor>,
        nombre: String,
        tipo_suscripcion: String,
        meses_contratados: u16,
    ) -> Result<()> {

        require!(
            context.accounts.plataforma.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

     msg!("Usuario {} dado de alta, con un plan {} por {} meses", nombre,  tipo_suscripcion,meses_contratados );

        let suscriptor = Suscriptor {
            nombre,
            tipo_suscripcion,
            meses_contratados,
            activa: true,
        };

        context.accounts.plataforma.suscriptores.push(suscriptor);

     


        Ok(())
    }


// endregion 
   
    // region: Función para ver Suscriptores

   pub fn ver_suscriptores(context: Context<NuevoSuscriptor>) -> Result<()> {

        require!(
            context.accounts.plataforma.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de suscriptores: {:#?}", context.accounts.plataforma.suscriptores);

        Ok(())
    }

// endregion

    // region: Función para buscar suscriptor por tipo

    pub fn buscar_por_tipo(
        context: Context<NuevoSuscriptor>,
        tipo: String,
    ) -> Result<()> {

        let suscriptores = &context.accounts.plataforma.suscriptores;

        let mut encontrados: Vec<String> = Vec::new();

        for i in 0..suscriptores.len() {
            if suscriptores[i].tipo_suscripcion == tipo {
                encontrados.push(suscriptores[i].nombre.clone());
            }
        }

        if encontrados.len() == 0 {
            msg!("No se encontraron suscriptores con la siguiente suscripcion: {}", tipo);
        } else {
            msg!("Suscriptores encontrados con tipo de suscripción {} : {:#?}, Lista de suscriptores {:#?}",tipo, encontrados.len(), encontrados);
        }

        Ok(())
    }

// endregion

    // region: Función para alternar estado de¿ la suscripción del suscriptor

pub fn alternar_estado_suscripcion(
    context: Context<NuevoSuscriptor>,
    nombre: String,
) -> Result<()> {

    require!(
        context.accounts.plataforma.owner == context.accounts.owner.key(),
        Errores::NoEresElOwner
    );

    let suscriptores = &mut context.accounts.plataforma.suscriptores;

    for i in 0..suscriptores.len() {

        if suscriptores[i].nombre == nombre {

            // Invertimos el estado actual
            let estado_actual = suscriptores[i].activa;
            let nuevo_estado = !estado_actual;

            suscriptores[i].activa = nuevo_estado;

            msg!(
                "La suscripción de {} cambio de {} a {}",nombre,estado_actual, nuevo_estado);

            return Ok(());
        }
    }
    Err(Errores::SuscriptorNoExiste.into())
}


// endregion

    // region: Función para eliminar suscriptor
  pub fn eliminar_suscriptor(
        context: Context<NuevoSuscriptor>,
        nombre: String,
    ) -> Result<()> {

        require!(
            context.accounts.plataforma.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let suscriptores = &mut context.accounts.plataforma.suscriptores;

        for i in 0..suscriptores.len() {
            if suscriptores[i].nombre == nombre {
                suscriptores.remove(i);
                msg!("Suscriptor {} eliminado exitosamente!", nombre);
                return Ok(());
            }
        }

        Err(Errores::SuscriptorNoExiste.into())
    }

// endregion

}


// region: Manejo de errores 

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la plataforma")]
    NoEresElOwner,

    #[msg("Error, el suscriptor no existe")]
    SuscriptorNoExiste,
}

// endregion


// region: Estructuras y contexto
#[account]
    // region: Struct de Plataforma
            #[derive(InitSpace)]
            pub struct Plataforma {
                owner: Pubkey,

                #[max_len(60)]
                nombre: String,

                #[max_len(50)]
                suscriptores: Vec<Suscriptor>,
            }
    // endregion

    // region: Struct de Suscriptor
        #[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
        pub struct Suscriptor {

            #[max_len(60)]
            nombre: String,

            #[max_len(30)]
            tipo_suscripcion: String,

            meses_contratados: u16,

            activa: bool,
        }
    // endregion

    // region: Struct de NuevaPlataforma
        #[derive(Accounts)]
        pub struct NuevaPlataforma<'info> {
            #[account(mut)]
            pub owner: Signer<'info>,

            #[account(
                init,
                payer = owner,
                space = Plataforma::INIT_SPACE + 8,
                seeds = [b"plataforma", owner.key().as_ref()],
                bump
            )]
            pub plataforma: Account<'info, Plataforma>,

            pub system_program: Program<'info, System>,
        }
    // endregion

    // region: Struct de NuevoSuscriptor
        #[derive(Accounts)]
        pub struct NuevoSuscriptor<'info> {
            pub owner: Signer<'info>,

            #[account(mut)]
            pub plataforma: Account<'info, Plataforma>,
        }
    // endregion

// endregion




 












