import pandas as pd
from pandas.testing import assert_frame_equal


def modifySalaryColumn(employees: pd.DataFrame) -> pd.DataFrame:
    employees["salary"] = employees["salary"] * 2
    return employees


def test_modifySalaryColumn():
    df = pd.DataFrame(
        {
            "name": ["Jack", "Piper", "Mia", "Ulysses"],
            "salary": [19666, 74754, 62509, 54866],
        }
    )
    got = modifySalaryColumn(df)
    expected = pd.DataFrame(
        {
            "name": ["Jack", "Piper", "Mia", "Ulysses"],
            "salary": [39332, 149508, 125018, 109732],
        }
    )
    assert_frame_equal(got, expected)
