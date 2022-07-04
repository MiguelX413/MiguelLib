from typing import List, Optional, Sequence, Tuple, Union

__version__: str

def match_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-8 indices of disjoint matches, from start to end."""

def match_utf16_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-16 indices of disjoint matches, from start to end."""

def match_byte_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the byte indices of disjoint matches, from start to end."""

def rmatch_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-8 indices of disjoint matches, from end to start."""

def rmatch_utf16_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the UTF-16 indices of disjoint matches, from end to start."""

def rmatch_byte_indices(string: str, substring: str) -> List[int]:
    """Returns a list of the byte indices of disjoint matches, from end to start."""

def utf16len(string: str) -> int:
    """A function that returns the UTF-16 length of a string."""

class Span:
    """A class used to represent spans."""

    @property
    def segments(self) -> List[Tuple[int, int]]: ...
    def __init__(self, segments: Optional[Sequence[Tuple[int, int]]] = ...) -> None: ...
    def copy(self) -> Span:
        """Return a shallow copy of a Span"""
    def difference(self, *others: Span) -> Span: ...
    def difference_update(self, *others: Span) -> None: ...
    def intersection(self, *others: Span) -> Span: ...
    def intersection_update(self, *others: Span) -> None: ...
    def isdisjoint(self, other: Span) -> bool:
        """Returns True if two Spans do not overlap."""
    def issubset(self, other: Span) -> bool:
        """Return True if other contains this Span, else False."""
    def issuperset(self, other: Span) -> bool:
        """Return True if this Span contains other, else False."""
    def union(self, *others: Span) -> Span: ...
    def union_update(self, *others: Span) -> None: ...
    def __or__(self, other: Span) -> Span: ...
    def __ior__(self, other: Span) -> None: ...
    def __and__(self, other: Span) -> Span: ...
    def __iand__(self, other: Span) -> None: ...
    def __sub__(self, other: Span) -> Span: ...
    def __isub__(self, other: Span) -> None: ...
    def __contains__(self, item: int) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: Span) -> bool: ...
    def __ne__(self, other: Span) -> bool: ...
    def __lt__(self, other: Span) -> bool: ...
    def __le__(self, other: Span) -> bool: ...
    def __gt__(self, other: Span) -> bool: ...
    def __ge__(self, other: Span) -> bool: ...
    __hash__: None  # type: ignore

class Interval:
    """A class used to represent intervals."""

    @property
    def segments(self) -> List[Tuple[bool, float, float, bool]]: ...
    def __init__(
        self,
        segments_or_span: Optional[
            Union[Sequence[Tuple[bool, float, float, bool]], Span]
        ] = ...,
    ) -> None: ...
    def copy(self) -> Interval:
        """Return a shallow copy of an Interval"""
    def difference(self, *others: Interval) -> Interval: ...
    def difference_update(self, *others: Interval) -> None: ...
    def intersection(self, *others: Interval) -> Interval: ...
    def intersection_update(self, *others: Interval) -> None: ...
    def isdisjoint(self, other: Interval) -> bool:
        """Returns True if two Intervals do not overlap."""
    def issubset(self, other: Interval) -> bool:
        """Return True if other contains this Interval, else False."""
    def issuperset(self, other: Interval) -> bool:
        """Return True if this Interval contains other, else False."""
    def union(self, *others: Interval) -> Interval: ...
    def union_update(self, *others: Interval) -> None: ...
    def __or__(self, other: Interval) -> Interval: ...
    def __ior__(self, other: Interval) -> None: ...
    def __and__(self, other: Interval) -> Interval: ...
    def __iand__(self, other: Interval) -> None: ...
    def __sub__(self, other: Interval) -> Interval: ...
    def __isub__(self, other: Interval) -> None: ...
    def __contains__(self, item: float) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: Interval) -> bool: ...
    def __ne__(self, other: Interval) -> bool: ...
    def __lt__(self, other: Interval) -> bool: ...
    def __le__(self, other: Interval) -> bool: ...
    def __gt__(self, other: Interval) -> bool: ...
    def __ge__(self, other: Interval) -> bool: ...
    __hash__: None  # type: ignore
