use anchor_lang::{prelude::*, solana_program};
use mpl_core::{
    ID as CORE_PROGRAM_ID, instructions::{UpdateV1Builder}
};

use crate::{error::MPLXCoreError, state::CollectionAuthority};

#[derive(Accounts)]
pub struct UpdateNft <'info> {
    #[account(
        mut,
        constraint = authority.key() == collection_authority.creator @ MPLXCoreError::NotAuthorized
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = asset.owner == &CORE_PROGRAM_ID
    )]
    /// CHECK: Validated by checking
    pub asset: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @ MPLXCoreError::InvalidCollection,
        constraint = !collection.data_is_empty() @ MPLXCoreError::CollectionNotInitialized
    )]
    /// CHECK: Validated by checking
    pub collection: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"collection_authority", collection.key().as_ref()],
        bump = collection_authority.bump
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: Validated by checking
    pub core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}

impl <'info> UpdateNft <'info> {
    pub fn update_nft (&mut self, new_name: String) -> Result<()> {
        let signers_seeds: &[&[&[u8]]] = &[&[
            b"collection_authority",
            &self.collection.key().to_bytes(),
            &[self.collection_authority.bump]
        ]];

        let accounts = UpdateV1Builder::new()
                                        .asset(self.asset.key())
                                        .collection(Some(self.collection.key()))
                                        .payer(self.authority.key())
                                        .authority(Some(self.collection_authority.key()))
                                        .system_program(self.system_program.key())
                                        .new_name(new_name)
                                        .instruction();

        let account_infos = [
            self.asset.to_account_info(),
            self.collection.to_account_info(),
            self.authority.to_account_info(),
            self.collection_authority.to_account_info(),
            self.system_program.to_account_info(),
        ];        

        solana_program::program::invoke_signed(
            &accounts, 
            &account_infos, 
            signers_seeds
        )?;


        Ok(())
    }
}