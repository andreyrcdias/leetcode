import pandas as pd
from pandas.testing import assert_frame_equal


def fillMissingValues(products: pd.DataFrame) -> pd.DataFrame:
    products["quantity"] = products["quantity"].fillna(0).astype(int)
    return products


def test_fillMissingValues():
    df = pd.DataFrame(
        {
            "name": ["Wristwatch", "WirelessEarbuds", "GolfClubs", "Printer"],
            "quantity": [None, None, 779, 849],
            "price": [135, 821, 9319, 3051],
        }
    )
    got = fillMissingValues(df)
    expected = pd.DataFrame(
        {
            "name": ["Wristwatch", "WirelessEarbuds", "GolfClubs", "Printer"],
            "quantity": [0, 0, 779, 849],
            "price": [135, 821, 9319, 3051],
        }
    )
    assert_frame_equal(got, expected)
