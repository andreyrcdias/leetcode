import pandas as pd
from pandas.testing import assert_frame_equal


def meltTables(report: pd.DataFrame) -> pd.DataFrame:
    return report.melt(
        id_vars=["product"],
        value_vars=[
            "quarter_1",
            "quarter_2",
            "quarter_3",
            "quarter_4",
        ],
        var_name="quarter",
        value_name="sales",
    )


def test_meltTables():
    df = pd.DataFrame(
        {
            "product": ["Umbrella", "SleepingBag"],
            "quarter_1": [417, 800],
            "quarter_2": [224, 936],
            "quarter_3": [379, 93],
            "quarter_4": [611, 875],
        }
    )
    got = meltTables(df)
    expected = pd.DataFrame(
        {
            "product": [
                "Umbrella",
                "SleepingBag",
                "Umbrella",
                "SleepingBag",
                "Umbrella",
                "SleepingBag",
                "Umbrella",
                "SleepingBag",
            ],
            "quarter": [
                "quarter_1",
                "quarter_1",
                "quarter_2",
                "quarter_2",
                "quarter_3",
                "quarter_3",
                "quarter_4",
                "quarter_4",
            ],
            "sales": [417, 800, 224, 936, 379, 93, 611, 875],
        }
    )
    assert_frame_equal(got, expected)
