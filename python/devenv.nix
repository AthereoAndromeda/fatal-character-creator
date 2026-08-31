{pkgs, ...}: {
  packages = with pkgs; [ruff];
  languages.python = {
    enable = true;
    venv.enable = true;
    # This is relative to the parent `devenv.nix`,
    # not this one
    directory = "python";
    lsp.package = pkgs.ty;
    uv = {
      enable = true;
      sync.enable = true;
    };
  };
}
