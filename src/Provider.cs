using System.Net;
using System.Threading.Tasks;

namespace DDNSUpdate
{
    public interface Provider
    {
        Task Update(string domain, string record, IPAddress ip);
    }
}
