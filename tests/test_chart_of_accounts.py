import unittest
from chart_of_accounts import ChartOfAccounts

class TestChartOfAccounts(unittest.TestCase):
    def test_empty_chart_of_accounts(self):
        coa = ChartOfAccounts()
        self.assertEqual(len(coa.accounts), 0)

if __name__ == '__main__':
    unittest.main()
