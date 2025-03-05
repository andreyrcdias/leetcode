import pandas as pd
from pandas.testing import assert_frame_equal


def dropDuplicateEmails(customers: pd.DataFrame) -> pd.DataFrame:
    return customers.drop_duplicates(subset=["email"]).reset_index(drop=True)


def test_dropDuplicateEmails():
    df1 = pd.DataFrame(
        {
            "customer_id": [1, 2, 3, 4, 5, 6],
            "name": [
                "Ella",
                "David",
                "Zachary",
                "Alice",
                "Finn",
                "Violet",
            ],
            "email": [
                "emily@example.com",
                "michael@example.com",
                "sarah@example.com",
                "john@example.com",
                "john@example.com",
                "alice@example.com",
            ],
        }
    )
    got = dropDuplicateEmails(df1)
    expected = pd.DataFrame(
        {
            "customer_id": [1, 2, 3, 4, 6],
            "name": [
                "Ella",
                "David",
                "Zachary",
                "Alice",
                "Violet",
            ],
            "email": [
                "emily@example.com",
                "michael@example.com",
                "sarah@example.com",
                "john@example.com",
                "alice@example.com",
            ],
        }
    )
    assert_frame_equal(got, expected)
