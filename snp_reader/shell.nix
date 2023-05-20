# shell.nix
{ pkgs ? import <nixpkgs> {} }:
let
  my-python-packages = ps: with ps; [
  wikitools
    # pandas
    # requests
    # other python packages
  ];
  my-python = pkgs.python2.withPackages my-python-packages;
in my-python.env
