fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Compiling proto files...");

    /*tonic_build::configure()
        .out_dir("src/generated")
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path("src/generated/private_file_descriptor_set.bin")
        .compile_protos(&[
            "proto/mlmbox/private/gen_event_img/gen_event_img.proto",
        ], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));*/

    /*tonic_build::configure()
        .out_dir("src/generated")
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .build_server(false)
        .build_client(false)
        .compile_protos(&[
            "proto/mlmbox/local/types/account.proto",
            "proto/mlmbox/local/types/asset.proto",
            "proto/mlmbox/local/types/asset_pair.proto",
            "proto/mlmbox/local/types/distribution.proto",
            "proto/mlmbox/local/types/error.proto",
            "proto/mlmbox/local/types/google_authenticator.proto",
            "proto/mlmbox/local/types/processing.proto",
            "proto/mlmbox/local/types/telegram_bot_events.proto",
            "proto/mlmbox/local/types/uploader.proto",
            "proto/mlmbox/local/types/wallet.proto",

            "proto/mlmbox/local/account/account.proto",
            "proto/mlmbox/local/app/app.proto",
            "proto/mlmbox/local/asset/asset.proto",
            "proto/mlmbox/local/asset/coin_market_cap.proto",
            "proto/mlmbox/local/asset_pair/asset_pair.proto",
            "proto/mlmbox/local/binary/binary.proto",
            "proto/mlmbox/local/distribution/distribution.proto",
            "proto/mlmbox/local/exchanger/exchanger.proto",
            "proto/mlmbox/local/finance/pool_hunter.proto",
            "proto/mlmbox/local/finance/wallet.proto",
            "proto/mlmbox/local/gift_shop/gift_shop.proto",
            "proto/mlmbox/local/google_authenticator/google_authenticator.proto",
            "proto/mlmbox/local/locale/locale.proto",
            "proto/mlmbox/local/matrix/matrix.proto",
            "proto/mlmbox/local/product/product.proto",
        ], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));*/

    tonic_build::configure()
        .out_dir("src/generated")
        // .type_attribute(".", "#[derive(serde::Serialize)]")
        .build_server(true)
        .build_client(false)
        .file_descriptor_set_path("src/generated/biconom_file_descriptor.bin")
        .compile_protos(&[
                "proto/biconom/types/account.proto",
                "proto/biconom/types/account_policy.proto",
                "proto/biconom/types/bonus.proto",
                "proto/biconom/types/bonus_policy.proto",
                "proto/biconom/types/boundary.proto",
                "proto/biconom/types/calculation.proto",
                "proto/biconom/types/community.proto",
                "proto/biconom/types/community_policy.proto",
                "proto/biconom/types/confirmation.proto",
                "proto/biconom/types/confirmation_policy.proto",
                "proto/biconom/types/currency.proto",
                "proto/biconom/types/currency_policy.proto",
                "proto/biconom/types/currency_pair.proto",
                "proto/biconom/types/currency_pair_policy.proto",
                "proto/biconom/types/distributor.proto",
                "proto/biconom/types/distributor_policy.proto",
                "proto/biconom/types/distributor_branch.proto",
                "proto/biconom/types/distributor_branch_policy.proto",
                "proto/biconom/types/google_authenticator.proto",
                "proto/biconom/types/google_authenticator_policy.proto",
                "proto/biconom/types/locale.proto",
                "proto/biconom/types/network.proto",
                "proto/biconom/types/network_policy.proto",
                "proto/biconom/types/network_account.proto",
                "proto/biconom/types/network_account_policy.proto",
                "proto/biconom/types/network_partition.proto",
                "proto/biconom/types/network_partition_policy.proto",
                "proto/biconom/types/referral_link.proto",
                "proto/biconom/types/referral_link_policy.proto",
                "proto/biconom/types/relationship.proto",
                "proto/biconom/types/rounding.proto",
                "proto/biconom/types/session.proto",
                "proto/biconom/types/session_policy.proto",
                "proto/biconom/types/slot.proto",
                "proto/biconom/types/slot_policy.proto",
                "proto/biconom/types/slot_branch.proto",
                "proto/biconom/types/slot_branch_policy.proto",
                "proto/biconom/types/sort.proto",
                "proto/biconom/types/subject.proto",
                "proto/biconom/types/trace.proto",
                "proto/biconom/types/tree.proto",
                "proto/biconom/types/tree_policy.proto",
                "proto/biconom/types/tree_distributor.proto",
                "proto/biconom/types/tree_distributor_policy.proto",
                "proto/biconom/types/tree_partition.proto",
                "proto/biconom/types/tree_partition_policy.proto",
                "proto/biconom/types/user.proto",
                "proto/biconom/types/user_policy.proto",

                "proto/biconom/client/auth/auth.proto",
                "proto/biconom/client/confirmation/confirmation.proto",
                "proto/biconom/client/currency/currency.proto",
                "proto/biconom/client/currency_pair/currency_pair.proto",
                "proto/biconom/client/google_authenticator/google_authenticator.proto",
                "proto/biconom/client/locale/locale.proto",
                "proto/biconom/client/referral_link/referral_link.proto",
                "proto/biconom/client/session/session.proto",
        ], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));

    println!("Proto files compiled successfully.");
    Ok(())
}
