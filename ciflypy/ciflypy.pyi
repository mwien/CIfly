from typing import List, Mapping, Set, Tuple, Protocol

class SupportsStr(Protocol):
    def __str__(self) -> str: ...

class Ruletable:
    def __init__(
        self, ruletable: SupportsStr, *, table_as_string: bool = False
    ) -> None:
        """
        Reads ruletable into CIfly outside of reach. Mostly recommended for improving performance if the same ruletable is used multiple times.

        Parameters
        ----------
        ruletable: Path to ruletable file.
        table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.

        Returns
        -------
        Internal CIfly representation of a ruletable. This object can be passed instead of a
        file path to all methods with a ruletable argument.
        """
        ...

class Graph:
    def __init__(
        self,
        graph: Mapping[str, List[Tuple[int, int]] | Set[Tuple[int, int]]],
        ruletable: SupportsStr | Ruletable,
        *,
        table_as_string: bool = False,
    ) -> None:
        """
        Reads graph into CIfly outside of reach. The parsed graph can be used in combination with all ruletables that have the same `EDGES ...` line as the passed ruletable argument. This method is mostly recommended for improving performance if the same graph is used multiple times.

        Parameters
        ----------
        graph: A dictionary mapping edge types to edge lists.
        ruletable: Path to ruletable file.
        table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.

        Returns
        -------
        Internal CIfly representation of a graph. This object can be passed instead of a dictionary mapping edge types to edge lists to all methods with a graph argument. It is compatible with all ruletables with the same `EDGES ...` line as the passed ruletable.
        """
        ...

class Sets:
    def __init__(
        self,
        sets: Mapping[str, int | List[int] | Set[int]],
        ruletable: SupportsStr | Ruletable,
        *,
        table_as_string: bool = False,
    ) -> None:
        """
        Reads sets into CIfly outside of reach. The parsed sets can be used in combination with all ruletables that have the same `SETS ...` line as the passed ruletable argument. This method is mostly recommended for improving performance if the same sets are used multiple times.

        Parameters
        ----------
        sets: A dictionary mapping set names to a list of elements.
        ruletable: Path to ruletable file.
        table_as_string: Optional keyword argument to enable passing the ruletable as multi-line string. Default value is False.

        Returns
        -------
        Internal CIfly representation of sets. This object can be passed instead of a dictionary mapping set names to lists to all methods with a sets argument. It is compatible with all ruletables with the same `SETS ...` line as the passed ruletable.
        """
        ...

def reach(
    graph: Mapping[str, List[Tuple[int, int]] | Set[Tuple[int, int]]] | Graph,
    sets: Mapping[str, int | List[int] | Set[int]] | Sets,
    ruletable: SupportsStr | Ruletable,
    *,
    table_as_string: bool = False,
    verbose: bool = False,
) -> List[int]:
    """
    Performs the CIfly algorithm specified in the passed ruletable.

    Parameters
    ----------
    graph: A dictionary mapping edge types to edge lists.
    sets: A dictionary mapping set names to a list of elements.
    ruletable: Path to the ruletable file.
    table_as_string: Enable passing the ruletable as multi-line string. Default value is False.
    verbose: Optional keyword argument to enable logging. Default value is False.

    Returns
    -------
    A list of all reachable nodes.
    """
    ...
