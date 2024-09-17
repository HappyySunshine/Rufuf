{
  description = "rust version";
  inputs = {
      nixpkgs.url =  "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs}: let 
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
   # pkgs2 = import nixpkgs2 {inherit system;};
    in{
        devShells."${system}".default = pkgs.mkShell{
           packages = [
                    pkgs.cargo
                    pkgs.rustc  # glibc
                    pkgs.cargo-expand
                    pkgs.rust-analyzer
                    ];        

           nativeBuildInputs =  [ pkgs.pkg-config  ];

           buildInputs = with pkgs;[
                openssl_legacy
            ];
       };

    };
}

