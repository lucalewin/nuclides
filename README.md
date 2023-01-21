# nuclides

`nuclides` is a rust library that provides an interface to the nuclide database by the IEAE.

If there is a new version of the table of nuclides, please open an isssue on GitHub.

- Livechart: Table of Nuclides (IAEA): <https://www-nds.iaea.org/relnsd/vcharthtml/VChartHTML.html>
- Nuclide API Reference (IAEA): <https://www-nds.iaea.org/relnsd/vcharthtml/api_v0_guide.html>

## Decay modes

- `N` = Neutrons
- `P` = Protons

| Mode | Name | Action | Nucleus changes |
|---|---|---|---|
| α | Alpha emission (2N + 2P) |  | N-2, P-2 |
| P | Proton emission | | N, P-1 |
| 2P | Double proton emission | | N, P-2 |
| N | Neutron emission | | N-1, P |
| 2N | Double neutron emission | | N-2, P |
| ε | Electron capture | | N+1, P-1 |
| e<sup>+</sup> | Positron emission | | N+1, P-1 |
| β<sup>+</sup> | Electron Capture + Positron emission (β<sup>+</sup> = ε + e<sup>+</sup>) | | N+1, P-1 |
| β<sup>−</sup> | Electron emission | | N-1, P+1 |
| 2β<sup>−</sup> | Double Electron emission | | N-2, P+2 |
| 2β<sup>+</sup> | Double Positron emission | | N+2, P-2 |
| | | | |
| SF | Spontaneus fission | | variable |
| STABLE | Stable | | N, P |

## Sources of the Data files

- table of nuclides: <https://www-nds.iaea.org/relnsd/v1/data?fields=ground_states&nuclides=all>
- periodic table: <https://pubchem.ncbi.nlm.nih.gov/periodic-table/>

## License

This project is licensed under the `BSD-2-Clause Plus Patent License` ([LICENSE](./LICENSE) or <https://opensource.org/licenses/BSDplusPatent>)
