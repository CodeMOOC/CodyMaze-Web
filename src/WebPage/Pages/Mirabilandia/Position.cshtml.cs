using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;

namespace WebPage.Pages
{
    public class PositionModel : PageModel
    {
        private readonly ILogger<PositionModel> _logger;

        public PositionModel(ILogger<PositionModel> logger)
        {
            _logger = logger;
        }

        public string? Code { get; set; }

        public void OnGet(string code)
        {
            Code = code?.ToUpperInvariant();
        }
    }
}
