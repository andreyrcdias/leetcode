import pandas as pd
from pandas.testing import assert_frame_equal


def selectFirstRows(employees: pd.DataFrame) -> pd.DataFrame:
    return employees.head(3)


def test_selectFirstRows():
    df = pd.DataFrame(
        {
            "employee_id": [3, 90, 9, 60, 49, 43],
            "name": ["Bob", "Alice", "Tatiana", "Annabelle", "Jonathan", "Khaled"],
            "department": [
                "Operations",
                "Sales",
                "Engineering",
                "InformationTechnology",
                "HumanResources",
                "Administration",
            ],
            "salary": [48675, 11096, 33805, 37678, 23793, 40454],
        }
    )
    got = selectFirstRows(df)
    expected = pd.DataFrame(
        {
            "employee_id": [3, 90, 9],
            "name": ["Bob", "Alice", "Tatiana"],
            "department": ["Operations", "Sales", "Engineering"],
            "salary": [48675, 11096, 33805],
        }
    )
    assert_frame_equal(got, expected)
