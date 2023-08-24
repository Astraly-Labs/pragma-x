pub const PRAGMA_CHAIN_SPEC_URL: &str = ""; // add Pragma chain spec url if not present in config folder

pub const GENESIS_ASSETS_URL: &str = ""; // add Pragma built genesis assets url if not present in config folder

pub const GENESIS_ASSETS_FILES: [&str; 1] = ["genesis.json"];

pub const CAIRO_CONTRACTS_URL: &str = ""; // add Pragma built contracts url if not present in config folder

pub const CAIRO_CONTRACTS_FILES: [&str; 7] = [
    "/configs/cairo-contracts/NoValidateAccount.json",
    "/configs/cairo-contracts/OpenzeppelinAccount.json",
    "/configs/cairo-contracts/ArgentAccount.json",
    "/configs/cairo-contracts/ERC20.json",
    "/configs/cairo-contracts/ERC721.json",
    "/configs/cairo-contracts/UniversalDeployer.json",
    "/configs/cairo-contracts/cairo_1/NoValidateAccount.casm.json",
];
