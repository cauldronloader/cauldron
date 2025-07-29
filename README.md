<div align="center">
	<h1 align="center"><b>Cauldron</b></h1>
	<p align="center">
		A Decima mod loader.
    </p>
<br/>

[![CI][sb0]][b0] [![CML][sb2]][b2] [![WORKSHOP][sb1]][b1]

![Alt](https://repobeats.axiom.co/api/embed/24e14d84adbd7b0e7c4640c172ad99c74a3de7d7.svg "Repobeats analytics image")

</div>

[sb0]: https://img.shields.io/github/actions/workflow/status/cauldron-decima/cauldron/build.yml?style=flat-square&logo=rust&logoColor=%23cad3f5&labelColor=%23363a4f&color=%23a6da95
[b0]: https://github.com/cauldron-decima/cauldron/actions/workflows/build.yml
[sb1]: https://img.shields.io/discord/1012475585605414983?style=flat-square&logo=discord&logoColor=%23cad3f5&labelColor=%23363a4f&color=%238aadf4&label=workshop
[b1]: https://discord.gg/Gt4gkMwadB
[sb2]: https://img.shields.io/discord/1393062934623682582?style=flat-square&logo=discord&logoColor=%23cad3f5&labelColor=%23363a4f&color=%238aadf4&label=cauldron
[b2]: https://discord.gg/AhDXJJxFsm

## Crates
- `cauldron` - The core mod loader api.
- `cauldron_config` - Common configuration across multiple crates.
- `cauldron_game_detection` - Game installation detection, using metadata like Steam's `libraryfolders.vdf`.
- `cauldron_loader` - The actual mod loader.
- `libdecima` - Includes types and addresses for supported games.
- `pulse` - Decima RTTI and symbol dumper in Cauldron mod form.
- `winhttp` - A proxy dll used for loading Cauldron itself.
- `heph` - The Cauldron build tool. Essentially an `xtask` with a different name.

## Credits
- [ShadelessFox](https://github.com/ShadelessFox) - For [Decima Workshop](https://github.com/ShadelessFox/decima) and the work they've put into reverse engineering the Decima engine.
- [Nukem9](https://github.com/Nukem9/) - For the help with the initial GGRTTI dumping and their work reverse engineering the Decima engine.
- [Fexty12573](https://github.com/Fexty12573/) - For the RTTI patching systems in their [DecimaLoader](https://github.com/Fexty12573/DecimaLoader/).
- [Guerrilla Games](https://www.guerrilla-games.com/) - For the Horizon series as a whole. 🧡

