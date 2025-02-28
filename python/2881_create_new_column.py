import pandas as pd
from pandas.testing import assert_frame_equal


def createBonusColumn(employees: pd.DataFrame) -> pd.DataFrame:
    employees["bonus"] = employees["salary"] * 2
    return employees


def test_createBonusColumn():
    df = pd.DataFrame(
        {
            "name": ["Piper", "Grace", "Georgia", "Willow", "Finn", "Thomas"],
            "salary": [4548, 28150, 1103, 6593, 74576, 24433],
        }
    )
    got = createBonusColumn(df)
    expected = pd.DataFrame(
        {
            "name": ["Piper", "Grace", "Georgia", "Willow", "Finn", "Thomas"],
            "salary": [4548, 28150, 1103, 6593, 74576, 24433],
            "bonus": [9096, 56300, 2206, 13186, 149152, 48866],
        }
    )
    assert_frame_equal(got, expected)
