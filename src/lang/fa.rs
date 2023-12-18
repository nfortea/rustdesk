lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "وضعیت"),
        ("Your Desktop", "دسکتاپ شما"),
        ("desk_tip", "دسکتاپ شما با این شناسه و رمز عبور قابل دسترسی است"),
        ("Password", "رمز عبور"),
        ("Ready", "آماده به کار"),
        ("Established", "اتصال برقرار شد"),
        ("connecting_status", "...در حال برقراری ارتباط با سرور"),
        ("Enable service", "فعالسازی سرویس"),
        ("Start service", "اجرای سرویس"),
        ("Service is running", "سرویس در حال اجرا است"),
        ("Service is not running", "سرویس اجرا نشده"),
        ("not_ready_status", "ارتباط برقرار نشد. لطفا شبکه خود را بررسی کنید"),
        ("Control Remote Desktop", "کنترل دسکتاپ میزبان"),
        ("Transfer file", "انتقال فایل"),
        ("Connect", "اتصال"),
        ("Recent sessions", "جلسات اخیر"),
        ("Address book", "دفترچه آدرس"),
        ("Confirmation", "تایید"),
        ("TCP tunneling", "TCP تانل"),
        ("Remove", "حذف"),
        ("Refresh random password", "بروزرسانی رمز عبور تصادفی"),
        ("Set your own password", "!رمز عبور دلخواه بگذارید"),
        ("Enable keyboard/mouse", " فعالسازی ماوس/صفحه کلید"),
        ("Enable clipboard", "فعال سازی کلیپبورد"),
        ("Enable file transfer", "انتقال فایل را فعال کنید"),
        ("Enable TCP tunneling", "را فعال کنید TCP تانل"),
        ("IP Whitelisting", "های مجاز IP لیست"),
        ("ID/Relay Server", "ID/Relay سرور"),
        ("Import server config", "تنظیم سرور با فایل"),
        ("Export Server Config", "ایجاد فایل تظیمات از سرور فعلی"),
        ("Import server configuration successfully", "تنظیمات سرور با فایل کانفیگ با موفقیت انجام شد"),
        ("Export server configuration successfully", "ایجاد فایل کانفیگ از تنظیمات فعلی با موفقیت انجام شد"),
        ("Invalid server configuration", "تنظیمات سرور نامعتبر است"),
        ("Clipboard is empty", "کلیپبورد خالی است"),
        ("Stop service", "توقف سرویس"),
        ("Change ID", "تعویض شناسه"),
        ("Your new ID", "جدید ID"),
        ("length %min% to %max%", "%max% تا %min% طول از"),
        ("starts with a letter", "با حرف شروع می شود"),
        ("allowed characters", "کارکترهای مجاز"),
        ("id_change_tip", "شناسه باید طبق این شرایط باشد : حروف کوچک و بزرگ انگلیسی و اعداد از 0 تا 9، _ و همچنین حرف اول آن فقط حروف بزرگ یا کوچک انگلیسی و طول آن بین 6 الی 16 کاراکتر باشد"),
        ("Website", "وب سایت"),
        ("About", "درباره"),
        ("Slogan_tip", "ساخته شده با قلب(عشق) در این دنیای پر هرج و مرج!"),
        ("Privacy Statement", "بیانیه حریم خصوصی"),
        ("Mute", "بستن صدا"),
        ("Build Date", "تاریخ ساخت"),
        ("Version", "نسخه"),
        ("Home", "صفحه اصلی"),
        ("Audio Input", "ورودی صدا"),
        ("Enhancements", "بهبودها"),
        ("Hardware Codec", "کدک سخت افزاری"),
        ("Adaptive bitrate", "سازگار Bitrate"),
        ("ID Server", "شناسه سرور"),
        ("Relay Server", "Relay سرور"),
        ("API Server", "API سرور"),
        ("invalid_http", "شروع شود http:// یا https:// باید با"),
        ("Invalid IP", "نامعتبر است IP آدرس"),
        ("Invalid format", "فرمت نادرست است"),
        ("server_not_support", "هنوز توسط سرور مورد نظر پشتیبانی نمی شود"),
        ("Not available", "در دسترسی نیست"),
        ("Too frequent", "خیلی رایج"),
        ("Cancel", "لغو"),
        ("Skip", "رد کردن"),
        ("Close", "بستن"),
        ("Retry", "تلاش مجدد"),
        ("OK", "قبول"),
        ("Password Required", "رمز عبور لازم است"),
        ("Please enter your password", "رمز عبور خود را وارد کنید"),
        ("Remember password", "رمز عبور را به خاطر بسپار"),
        ("Wrong Password", "رمز عبور اشتباه است"),
        ("Do you want to enter again?", "آیا میخواهید مجددا وارد شوید؟"),
        ("Connection Error", "خطا در اتصال"),
        ("Error", "خطا"),
        ("Reset by the peer", "توسط میزبان حذف شد"),
        ("Connecting...", "...در حال اتصال"),
        ("Connection in progress. Please wait.", "در حال اتصال. لطفا متظر بمانید"),
        ("Please try 1 minute later", "لطفا بعد از 1 دقیقه مجددا تلاش کنید"),
        ("Login Error", "ورود ناموفق بود"),
        ("Successful", "با موفقیت انجام شد"),
        ("Connected, waiting for image...", "...ارتباط برقرار شد. انتظار برای دریافت تصاویر"),
        ("Name", "نام"),
        ("Type", "نوع فایل"),
        ("Modified", "تاریخ تغییر"),
        ("Size", "سایز"),
        ("Show Hidden Files", "نمایش فایل های مخفی"),
        ("Receive", "دریافت"),
        ("Send", "ارسال"),
        ("Refresh File", "به روزرسانی فایل"),
        ("Local", "محلی"),
        ("Remote", "از راه دور"),
        ("Remote Computer", "سیستم میزبان"),
        ("Local Computer", "سیستم راه دور"),
        ("Confirm Delete", "تایید حذف"),
        ("Delete", "حذف"),
        ("Properties", "مشخصات"),
        ("Multi Select", "انتخاب دسته ای"),
        ("Select All", "انتخاب همه"),
        ("Unselect All", "لغو انتخاب همه"),
        ("Empty Directory", "پوشه خالی"),
        ("Not an empty directory", "پوشه خالی نیست"),
        ("Are you sure you want to delete this file?", "از حذف این فایل مطمئن هستید؟"),
        ("Are you sure you want to delete this empty directory?", "از حذف این پوشه خالی مطمئن هستید؟"),
        ("Are you sure you want to delete the file of this directory?", "از حذف فایل موجود در این پوشه مطمئن هستید؟"),
        ("Do this for all conflicts", "این عمل برای همه ی تضادها انجام شود"),
        ("This is irreversible!", "این اقدام برگشت ناپذیر است!"),
        ("Deleting", "در حال حذف"),
        ("files", "فایل ها"),
        ("Waiting", "انتظار"),
        ("Finished", "تکمیل شد"),
        ("Speed", "سرعت"),
        ("Custom Image Quality", "سفارشی سازی کیفیت تصاویر"),
        ("Privacy mode", "حالت حریم خصوصی"),
        ("Block user input", "بلاک کردن ورودی کاربر"),
        ("Unblock user input", "آنبلاک کردن ورودی کاربر"),
        ("Adjust Window", "تنظیم پنجره"),
        ("Original", "اصل"),
        ("Shrink", "کوچک کردن"),
        ("Stretch", "کشیدن تصویر"),
        ("Scrollbar", "اسکرول بار"),
        ("ScrollAuto", "پیمایش/اسکرول خودکار"),
        ("Good image quality", "کیفیت خوب تصویر"),
        ("Balanced", "متعادل"),
        ("Optimize reaction time", "بهینه سازی زمان واکنش"),
        ("Custom", "سفارشی"),
        ("Show remote cursor", "نمایش مکان نما موس میزبان"),
        ("Show quality monitor", "نمایش کیفیت مانیتور"),
        ("Disable clipboard", " غیرفعالسازی کلیپبورد"),
        ("Lock after session end", "قفل کردن حساب کاربری سیستم عامل پس از پایان جلسه"),
        ("Insert", "افزودن"),
        ("Insert Lock", "افزودن قفل"),
        ("Refresh", "تازه سازی"),
        ("ID does not exist", "شناسه وجود ندارد"),
        ("Failed to connect to rendezvous server", "اتصال به سرور تولید شناسه انجام نشد"),
        ("Please try later", "لطفا بعدا تلاش کنید"),
        ("Remote desktop is offline", "دسکتاپ راه دور آفلاین است"),
        ("Key mismatch", "عدم تطابق کلید"),
        ("Timeout", "زمان انتظار به پایان رسید"),
        ("Failed to connect to relay server", "سرور وصل نشد Relay به"),
        ("Failed to connect via rendezvous server", "اتصال از طریق سرور تولید شناسه انجام نشد"),
        ("Failed to connect via relay server", "انجام نشد Relay اتصال از طریق سرور"),
        ("Failed to make direct connection to remote desktop", "اتصال مستقیم به دسکتاپ راه دور انجام نشد"),
        ("Set Password", "تنظیم رمزعبور"),
        ("OS Password", "رمز عبور سیستم عامل"),
        ("install_tip", "لطفا برنامه را نصب کنید UAC و جلوگیری از خطای StarDesk برای راحتی در استفاده از نرم افزار"),
        ("Click to upgrade", "برای ارتقا کلیک کنید"),
        ("Click to download", "برای دانلود کلیک کنید"),
        ("Click to update", "برای به روز رسانی کلیک کنید"),
        ("Configure", "تنظیم"),
        ("config_acc", "بدهید \"access\" مجوز StarDesk برای کنترل از راه دور دسکتاپ باید به"),
        ("config_screen", "بدهید \"screenshot\" مجوز StarDesk برای کنترل از راه دور دسکتاپ باید به"),
        ("Installing ...", "...در حال نصب"),
        ("Install", "نصب"),
        ("Installation", "نصب و راه اندازی"),
        ("Installation Path", "محل نصب"),
        ("Create start menu shortcuts", "Start ایجاد میانبرها در منوی"),
        ("Create desktop icon", "ایجاد آیکن در دسکتاپ"),
        ("agreement_tip", "با شروع نصب، شرایط توافق نامه مجوز را می پذیرید"),
        ("Accept and Install", "قبول و شروع نصب"),
        ("End-user license agreement", "قرارداد مجوز کاربر نهایی"),
        ("Generating ...", "...در حال تولید"),
        ("Your installation is lower version.", "نسخه قدیمی تری نصب شده است"),
        ("not_close_tcp_tip", "هنگام استفاده از تونل این پنجره را نبندید"),
        ("Listening ...", "...انتظار"),
        ("Remote Host", "هاست راه دور"),
        ("Remote Port", "پورت راه دور"),
        ("Action", "عملیات"),
        ("Add", "افزودن"),
        ("Local Port", "پورت محلی"),
        ("Local Address", "آدرس محلی"),
        ("Change Local Port", "تغییر پورت محلی"),
        ("setup_server_tip", "برای اتصال سریعتر، سرور اتصال شخصی خود را راه اندازی کنید"),
        ("Too short, at least 6 characters.", "بسیار کوتاه حداقل 6 کاراکتر مورد نیاز است"),
        ("The confirmation is not identical.", "تأیید ناموفق بود."),
        ("Permissions", "دسترسی ها"),
        ("Accept", "پذیرفتن"),
        ("Dismiss", "رد کردن"),
        ("Disconnect", "قطع اتصال"),
        ("Enable file copy and paste", "مجاز بودن کپی و چسباندن فایل"),
        ("Connected", "متصل شده"),
        ("Direct and encrypted connection", "اتصال مستقیم و رمزگذاری شده"),
        ("Relayed and encrypted connection", "و رمزگذاری شده Relay اتصال از طریق"),
        ("Direct and unencrypted connection", "اتصال مستقیم و بدون رمزگذاری"),
        ("Relayed and unencrypted connection", "و رمزگذاری نشده Relay اتصال از طریق"),
        ("Enter Remote ID", "شناسه از راه دور را وارد کنید"),
        ("Enter your password", "زمر عبور خود را وارد کنید"),
        ("Logging in...", "...در حال ورود"),
        ("Enable RDP session sharing", "را فعال کنید RDP اشتراک گذاری جلسه"),
        ("Auto Login", "ورود خودکار"),
        ("Enable direct IP access", "را فعال کنید IP دسترسی مستقیم"),
        ("Rename", "تغییر نام"),
        ("Space", "فضا"),
        ("Create desktop shortcut", "ساخت میانبر روی دسکتاپ"),
        ("Change Path", "تغییر مسیر"),
        ("Create Folder", "ایجاد پوشه"),
        ("Please enter the folder name", "نام پوشه را وارد کنید"),
        ("Fix it", "بازسازی"),
        ("Warning", "هشدار"),
        ("Login screen using Wayland is not supported", "پشتیبانی نمی شود Wayland ورود به سیستم با استفاده از "),
        ("Reboot required", "راه اندازی مجدد مورد نیاز است"),
        ("Unsupported display server", "سرور تصویر پشتیبانی نشده است"),
        ("x11 expected", ""),
        ("Port", "پورت"),
        ("Settings", "تنظیمات"),
        ("Username", "نام کاربری"),
        ("Invalid port", "پورت نامعتبر است"),
        ("Closed manually by the peer", "به صورت دستی توسط میزبان بسته شد"),
        ("Enable remote configuration modification", "فعال بودن اعمال تغییرات پیکربندی از راه دور"),
        ("Run without install", "بدون نصب اجرا شود"),
        ("Connect via relay", "اتصال با رله"),
        ("Always connect via relay", "برای اتصال استفاده شود Relay از"),
        ("whitelist_tip", "های مجاز می توانند به این دسکتاپ متصل شوند IP فقط"),
        ("Login", "ورود"),
        ("Verify", "تأیید کنید"),
        ("Remember me", "مرا به یاد داشته باش"),
        ("Trust this device", "به این دستگاه اعتماد کنید"),
        ("Verification code", "کد تایید"),
        ("verification_tip", "یک دستگاه جدید شناسایی شده است و یک کد تأیید به آدرس ایمیل ثبت شده ارسال شده است، برای ادامه ورود، کد تأیید را وارد کنید."),
        ("Logout", "خروج"),
        ("Tags", "برچسب ها"),
        ("Search ID", "جستجوی شناسه"),
        ("whitelist_sep", "با کاما، نقطه ویرگول، فاصله یا خط جدید از هم جدا می شوند"),
        ("Add ID", "افزودن شناسه"),
        ("Add Tag", "افزودن برچسب"),
        ("Unselect all tags", "همه برچسب ها را لغو انتخاب کنید"),
        ("Network error", "خطای شبکه"),
        ("Username missed", "نام کاربری وجود ندارد"),
        ("Password missed", "رمزعبور وجود ندارد"),
        ("Wrong credentials", "اعتبارنامه نادرست است"),
        ("The verification code is incorrect or has expired", "کد تأیید نادرست است یا منقضی شده است"),
        ("Edit Tag", "ویرایش برچسب"),
        ("Forget Password", "رمز عبور ذخیره نشود"),
        ("Favorites", "اتصالات دلخواه"),
        ("Add to Favorites", "افزودن به علاقه مندی ها"),
        ("Remove from Favorites", "از علاقه مندی ها حذف شود"),
        ("Empty", "موردی وجود ندارد"),
        ("Invalid folder name", "نام پوشه نامعتبر است"),
        ("Socks5 Proxy", "Socks5 پروکسی"),
        ("Hostname", "نام هاست"),
        ("Discovered", "پیدا شده"),
        ("install_daemon_tip", "برای شروع در هنگام راه اندازی، باید سرویس سیستم را نصب کنید"),
        ("Remote ID", "شناسه راه دور"),
        ("Paste", "درج"),
        ("Paste here?", "اینجا درج شود؟"),
        ("Are you sure to close the connection?", "آیا مطمئن هستید که می خواهید اتصال را پایان دهید؟"),
        ("Download new version", "دانلود نسخه جدید"),
        ("Touch mode", "حالت لمسی"),
        ("Mouse mode", "حالت ماوس"),
        ("One-Finger Tap", "با یک انگشت لمس کنید"),
        ("Left Mouse", "دکمه سمت چپ ماوس"),
        ("One-Long Tap", "لمس طولانی با یک انگشت"),
        ("Two-Finger Tap", "لمس دو انگشتی"),
        ("Right Mouse", "دکمه سمت راست ماوس"),
        ("One-Finger Move", "با یک انگشت حرکت کنید"),
        ("Double Tap & Move", "دو ضربه سریع بزنید و حرکت دهید"),
        ("Mouse Drag", "کشیدن ماوس"),
        ("Three-Finger vertically", "سه انگشت عمودی"),
        ("Mouse Wheel", "چرخ ماوس"),
        ("Two-Finger Move", "با دو انگشت حرکت کنید"),
        ("Canvas Move", ""),
        ("Pinch to Zoom", "با دو انگشت بکشید تا زوم شود"),
        ("Canvas Zoom", ""),
        ("Reset canvas", ""),
        ("No permission of file transfer", "مجوز انتقال فایل داده نشده"),
        ("Note", "یادداشت"),
        ("Connection", "ارتباط"),
        ("Share Screen", "اشتراک گذاری صفحه"),
        ("Chat", "چت"),
        ("Total", "مجموع"),
        ("items", "آیتم ها"),
        ("Selected", "انتخاب شده"),
        ("Screen Capture", "ضبط صفحه"),
        ("Input Control", "کنترل ورودی"),
        ("Audio Capture", "ضبط صدا"),
        ("File Connection", "ارتباط فایل"),
        ("Screen Connection", "ارتباط صفحه"),
        ("Do you accept?", "آیا می پذیرید؟"),
        ("Open System Setting", "باز کردن تنظیمات سیستم"),
        ("How to get Android input permission?", "چگونه مجوز ورود به سیستم اندروید را دریافت کنیم؟"),
        ("android_input_permission_tip1", "استفاده کند \"Accessibility\" اجازه دهید از ویژگی StarDesk شما را از طریق ماوس یا صفحه ی لمسی کنترل کند، باید به Android برای اینکه یک دستگاه از راه دور بتواند دستگاه"),
        ("android_input_permission_tip2", "را پیدا کنید و فعال نمایید \"StarDesk Input\" بشوید ، سپس گزینه \"Installed Services\" وارد قسمت \"Accessibility\" در صفحه تنظیمات "),
        ("android_new_connection_tip", "درخواست جدیدی برای مدیریت دستگاه فعلی شما دریافت شده است."),
        ("android_service_will_start_tip", "فعال کردن ضبط صفحه به طور خودکار سرویس را راه اندازی می کند و به دستگاه های دیگر امکان می دهد درخواست اتصال به آن دستگاه را داشته باشند."),
        ("android_stop_service_tip", "با بستن سرویس، تمام اتصالات برقرار شده به طور خودکار بسته می شود"),
        ("android_version_audio_tip", "نسخه فعلی اندروید از ضبط صدا پشتیبانی نمی‌کند، لطفاً به اندروید 10 یا بالاتر به‌روزرسانی کنید"),
        ("android_start_service_tip", "را فعال کنید [Screen Capture] ضربه بزنید یا مجوز [Start service] برای شروع سرویس اشتراک ‌گذاری صفحه، روی"),
        ("android_permission_may_not_change_tip", "مجوزهای ایجاد شده یا تغییر یافته برای اتصالات جاری تغییر نخواهد کرد، برای تغییر نیاز است مجددا اتصال برقرار گردد"),
        ("Account", "حساب کاربری"),
        ("Overwrite", "بازنویسی"),
        ("This file exists, skip or overwrite this file?", "این فایل وجود دارد، از فایل رد شود یا آن را بازنویسی کند؟"),
        ("Quit", "خروج"),
        ("Help", "راهنما"),
        ("Failed", "ناموفق"),
        ("Succeeded", "موفقیت آمیز"),
        ("Someone turns on privacy mode, exit", "اگر شخصی حالت حریم خصوصی را روشن کرد، خارج شوید"),
        ("Unsupported", "پشتیبانی نشده"),
        ("Peer denied", "توسط میزبان راه دور رد شد"),
        ("Please install plugins", "لطفا افزونه ها را نصب کنید"),
        ("Peer exit", "میزبان خارج شد"),
        ("Failed to turn off", "خاموش کردن انجام نشد"),
        ("Turned off", "خاموش شد"),
        ("Language", "زبان"),
        ("Keep StarDesk background service", "را در پس زمینه نگه دارید StarDesk سرویس"),
        ("Ignore Battery Optimizations", "بهینه سازی باتری نادیده گرفته شود"),
        ("android_open_battery_optimizations_tip", "به صفحه تنظیمات بعدی بروید"),
        ("Start on boot", "در هنگام بوت شروع شود"),
        ("Start the screen sharing service on boot, requires special permissions", "سرویس اشتراک‌گذاری صفحه را در بوت راه‌اندازی کنید، به مجوزهای خاصی نیاز دارد"),
        ("Connection not allowed", "اتصال مجاز نیست"),
        ("Legacy mode", "legacy حالت"),
        ("Map mode", "map حالت"),
        ("Translate mode", "حالت ترجمه"),
        ("Use permanent password", "از رمز عبور دائمی استفاده شود"),
        ("Use both passwords", "از هر دو رمز عبور استفاده شود"),
        ("Set permanent password", "یک رمز عبور دائمی تنظیم شود"),
        ("Enable remote restart", "فعال کردن قابلیت ریستارت از راه دور"),
        ("Restart remote device", "ریستارت کردن از راه دور"),
        ("Are you sure you want to restart", "ایا مطمئن هستید میخواهید راه اندازی مجدد انجام بدید؟"),
        ("Restarting remote device", "در حال راه اندازی مجدد دستگاه راه دور"),
        ("remote_restarting_tip", "دستگاه راه دور در حال راه اندازی مجدد است. این پیام را ببندید و پس از مدتی با استفاده از یک رمز عبور دائمی دوباره وصل شوید."),
        ("Copied", "کپی شده است"),
        ("Exit Fullscreen", "از حالت تمام صفحه خارج شوید"),
        ("Fullscreen", "تمام صفحه"),
        ("Mobile Actions", "اقدامات موبایل"),
        ("Select Monitor", "مانیتور را انتخاب کنید"),
        ("Control Actions", "اقدامات مدیریتی"),
        ("Display Settings", "تنظیمات نمایشگر"),
        ("Ratio", "نسبت"),
        ("Image Quality", "کیفیت تصویر"),
        ("Scroll Style", "سبک اسکرول"),
        ("Show Toolbar", "نمایش نوار ابزار"),
        ("Hide Toolbar", "پنهان کردن نوار ابزار"),
        ("Direct Connection", "ارتباط مستقیم"),
        ("Relay Connection", "Relay ارتباط"),
        ("Secure Connection", "ارتباط امن"),
        ("Insecure Connection", "ارتباط غیر امن"),
        ("Scale original", "مقیاس اصلی"),
        ("Scale adaptive", "مقیاس تطبیقی"),
        ("General", "عمومی"),
        ("Security", "امنیت"),
        ("Theme", "نمایه"),
        ("Dark Theme", "نمایه تیره"),
        ("Light Theme", "نمایه روشن"),
        ("Dark", "تیره"),
        ("Light", "روشن"),
        ("Follow System", "پیروی از سیستم"),
        ("Enable hardware codec", "فعال سازی کدک سخت افزاری"),
        ("Unlock Security Settings", "دسترسی کامل به تنظیمات امنیتی"),
        ("Enable audio", "فعال شدن صدا"),
        ("Unlock Network Settings", "دسترسی کامل به تنظیمات شبکه"),
        ("Server", "سرور"),
        ("Direct IP Access", "IP دسترسی مستقیم به"),
        ("Proxy", "پروکسی"),
        ("Apply", "ثبت"),
        ("Disconnect all devices?", "همه دستگاه ها قطع شوند؟"),
        ("Clear", "پاک کردن"),
        ("Audio Input Device", "منبع صدا"),
        ("Use IP Whitelisting", "های مجاز IP استفاده از"),
        ("Network", "شبکه"),
        ("Pin Toolbar", "سجاق کردن نوار ابزار"),
        ("Unpin Toolbar", "خروج از حالت سجاق نوار ابزار"),
        ("Recording", "در حال ضبط"),
        ("Directory", "مسیر"),
        ("Automatically record incoming sessions", "ضبط خودکار جلسات ورودی"),
        ("Change", "تغییر"),
        ("Start session recording", "شروع ضبط جلسه"),
        ("Stop session recording", "توقف ضبط جلسه"),
        ("Enable recording session", "فعالسازی ضبط جلسه"),
        ("Enable LAN discovery", "فعالسازی جستجو در شبکه"),
        ("Deny LAN discovery", "غیر فعالسازی جستجو در شبکه"),
        ("Write a message", "یک پیام بنویسید"),
        ("Prompt", "سریع"),
        ("Please wait for confirmation of UAC...", "باشید UAC لطفا منتظر تایید"),
        ("elevated_foreground_window_tip", "پنجره فعلی دسکتاپ راه دور برای کار کردن به دسترسی بالاتری نیاز دارد، بنابراین نمی‌تواند به طور موقت از ماوس و صفحه کلید استفاده کند. می توانید از کاربر راه دور درخواست کنید که پنجره فعلی را به پایین منتقل کند یا روی دکمه ارتقاء دسترسی در پنجره مدیریت اتصال کلیک کنید. برای جلوگیری از این مشکل، توصیه می شود نرم افزار را روی دستگاه از راه دور نصب کنید."),
        ("Disconnected", "قطع ارتباط"),
        ("Other", "سایر"),
        ("Confirm before closing multiple tabs", "تایید بستن دسته ای برگه ها"),
        ("Keyboard Settings", "تنظیمات صفحه کلید"),
        ("Full Access", "دسترسی کامل"),
        ("Screen Share", "اشتراک گذاری صفحه"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "نیازمند اوبونتو نسخه 21.04 یا بالاتر است Wayland"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "استفاده کنید و یا سیستم عامل خود را تغییر دهید X11 نیازمند نسخه بالاتری از توزیع لینوکس است. لطفا از دسکتاپ با سیستم"),
        ("JumpLink", "چشم انداز"),
        ("Please Select the screen to be shared(Operate on the peer side).", "لطفاً صفحه‌ای را برای اشتراک‌گذاری انتخاب کنید (در سمت همتا به همتا کار کنید)."),
        ("Show StarDesk", "StarDesk نمایش"),
        ("This PC", "This PC"),
        ("or", "یا"),
        ("Continue with", "ادامه با"),
        ("Elevate", "ارتقاء"),
        ("Zoom cursor", " بزرگنمایی نشانگر ماوس"),
        ("Accept sessions via password", "قبول درخواست با رمز عبور"),
        ("Accept sessions via click", "قبول درخواست با کلیک موس"),
        ("Accept sessions via both", "قبول درخواست با هر دو"),
        ("Please wait for the remote side to accept your session request...", "...لطفا صبر کنید تا میزبان درخواست شما را قبول کند"),
        ("One-time Password", "رمز عبور یکبار مصرف"),
        ("Use one-time password", "استفاده از رمز عبور یکبار مصرف"),
        ("One-time password length", "طول رمز عبور یکبار مصرف"),
        ("Request access to your device", "دسترسی به دستگاه خود را درخواست کنید"),
        ("Hide connection management window", "پنهان کردن پنجره مدیریت اتصال"),
        ("hide_cm_tip", "فقط در صورت پذیرفتن جلسات از طریق رمز عبور و استفاده از رمز عبور دائمی، مخفی شدن مجاز است"),
        ("wayland_experiment_tip", "پشتیبانی Wayland در مرحله آزمایشی است، لطفاً در صورت نیاز به دسترسی بدون مراقبت از X11 استفاده کنید."),
        ("Right click to select tabs", "برای انتخاب تب ها راست کلیک کنید"),
        ("Skipped", "رد شد"),
        ("Add to address book", "افزودن به دفترچه آدرس"),
        ("Group", "گروه"),
        ("Search", "جستجو"),
        ("Closed manually by web console", "به صورت دستی توسط کنسول وب بسته شد"),
        ("Local keyboard type", "نوع صفحه کلید محلی"),
        ("Select local keyboard type", "نوع صفحه کلید محلی را انتخاب کنید"),
        ("software_render_tip", "اگر کارت گرافیک Nvidia دارید و پنجره راه دور بلافاصله پس از اتصال بسته می شود، درایور nouveau را نصب نمایید و انتخاب گزینه استفاده از رندر نرم افزار می تواند کمک کننده باشد. راه اندازی مجدد نرم افزار مورد نیاز است."),
        ("Always use software rendering", "همیشه از رندر نرم افزاری استفاده کنید"),
        ("config_input", "برای کنترل دسکتاپ از راه دور با صفحه کلید، باید مجوز StarDesk \"Input Monitoring\" را بدهید."),
        ("config_microphone", "را بدهید. StarDesk \"Record Audio\" برای صحبت از راه دور، باید مجوز"),
        ("request_elevation_tip", "همچنین می توانید در صورت وجود شخصی در سمت راه دور درخواست ارتقاء دسترسی دهید."),
        ("Wait", "صبر کنید"),
        ("Elevation Error", "خطای ارتقاء دسترسی"),
        ("Ask the remote user for authentication", "درخواست احراز هویت از یک کاربر راه دور"),
        ("Choose this if the remote account is administrator", "اگر حساب راه دور یک مدیر است، این را انتخاب کنید"),
        ("Transmit the username and password of administrator", "نام کاربری و رمز عبور مدیر را منتقل کنید"),
        ("still_click_uac_tip", "همچنان کاربر از راه دور نیاز دارد که روی OK در پنجره UAC اجرای StarDesk کلیک کند."),
        ("Request Elevation", "درخواست ارتقاء دسترسی"),
        ("wait_accept_uac_tip", "لطفاً منتظر بمانید تا کاربر راه دور درخواست پنجره UAC را بپذیرد."),
        ("Elevate successfully", "ارتقاء دسترسی با موفقیت انجام شد"),
        ("uppercase", "حروف بزرگ"),
        ("lowercase", "حروف کوچک"),
        ("digit", "عدد"),
        ("special character", "کاراکتر خاص"),
        ("length>=8", "حداقل طول 8 کاراکتر"),
        ("Weak", "ضعیف"),
        ("Medium", "متوسط"),
        ("Strong", "قوی"),
        ("Switch Sides", "طرفین را عوض کنید"),
        ("Please confirm if you want to share your desktop?", "لطفاً تأیید کنید که آیا می خواهید دسکتاپ خود را به اشتراک بگذارید؟"),
        ("Display", "نمایش دادن"),
        ("Default View Style", "سبک نمایش پیش فرض"),
        ("Default Scroll Style", "سبک پیش‌ فرض اسکرول"),
        ("Default Image Quality", "کیفیت تصویر پیش فرض"),
        ("Default Codec", "کدک پیش فرض"),
        ("Bitrate", "میزان بیت صفحه نمایش"),
        ("FPS", "FPS"),
        ("Auto", "خودکار"),
        ("Other Default Options", "سایر گزینه های پیش فرض"),
        ("Voice call", "تماس صوتی"),
        ("Text chat", "گفتگو متنی (چت متنی)"),
        ("Stop voice call", "توقف تماس صوتی"),
        ("relay_hint_tip", " را به شناسه اضافه کنید یا گزینه \"همیشه از طریق رله متصل شوید\" را در کارت همتا انتخاب کنید. همچنین، اگر می‌خواهید فوراً از سرور رله استفاده کنید، می‌توانید پسوند \"/r\".\n اتصال مستقیم ممکن است امکان پذیر نباشد. در این صورت می توانید سعی کنید از طریق سرور رله متصل شوید"),
        ("Reconnect", "اتصال مجدد"),
        ("Codec", "کدک"),
        ("Resolution", "وضوح"),
        ("No transfers in progress", "هیچ انتقالی در حال انجام نیست"),
        ("Set one-time password length", "طول رمز یکبار مصرف را تعیین کنید"),
        ("install_cert_tip", "StarDesk نصب گواهی"),
        ("confirm_install_cert_tip", "استفاده خواهد شد StarDesk است و شما می توانید به این گواهی اعتماد کنید. این گواهی برای اعتماد و نصب درایورهای StarDesk این گواهینامه یک گواهی تست"),
        ("RDP Settings", "RDP تنظیمات"),
        ("Sort by", "مرتب سازی بر اساس"),
        ("New Connection", "اتصال جدید"),
        ("Restore", "بازیابی"),
        ("Minimize", "کوچک کردن پنجره"),
        ("Maximize", "بزرک کردن پنجره"),
        ("Your Device", "دستگاه شما"),
        ("empty_recent_tip", "اوه، هیچ جلسه اخیری وجود ندارد!\nزمان برنامه ریزی جلسه جدید است"),
        ("empty_favorite_tip", "هنوز همتای مورد علاقه‌ای ندارید؟\nبیایید فردی را برای ارتباط پیدا کنیم و آن را به موارد دلخواه خود اضافه کنیم!"),
        ("empty_lan_tip", "اوه نه، به نظر می رسد که ما هنوز همتای خود را پیدا نکرده ایم"),
        ("empty_address_book_tip", "اوه ، به نظر می رسد که در حال حاضر هیچ همتایی در دفترچه آدرس شما وجود ندارد"),
        ("eg: admin", "مثال : admin"),
        ("Empty Username", "نام کاربری خالی است"),
        ("Empty Password", "رمز عبور خالی است"),
        ("Me", "من"),
        ("identical_file_tip", "این فایل با فایل همتا یکسان است."),
        ("show_monitors_tip", "نمایش مانیتورها در نوار ابزار"),
        ("View Mode", "حالت مشاهده"),
        ("login_linux_tip", "برای فعال کردن دسکتاپ X، باید به حساب لینوکس راه دور وارد شوید"),
        ("verify_StarDesk_password_tip", "رمز عبور StarDesk را تأیید کنید"),
        ("remember_account_tip", "این حساب را به خاطر بسپارید"),
        ("os_account_desk_tip", "این حساب برای ورود به سیستم عامل راه دور و فعال کردن جلسه دسکتاپ در هدلس استفاده می شود"),
        ("OS Account", "حساب کاربری سیستم عامل"),
        ("another_user_login_title_tip", "کاربر دیگری قبلاً وارد شده است"),
        ("another_user_login_text_tip", "قطع شدن"),
        ("xorg_not_found_title_tip", "پیدا نشد Xorg"),
        ("xorg_not_found_text_tip", "لطفا Xorg را نصب کنید"),
        ("no_desktop_title_tip", "هیچ دسکتاپی در دسترس نیست"),
        ("no_desktop_text_tip", "لطفا دسکتاپ گنوم را نصب کنید"),
        ("No need to elevate", "نیازی به ارتقاء نیست"),
        ("System Sound", "صدای سیستم"),
        ("Default", "پیش فرض"),
        ("New RDP", "ریموت جدید"),
        ("Fingerprint", "اثر انگشت"),
        ("Copy Fingerprint", "کپی کردن اثر انگشت"),
        ("no fingerprints", "بدون اثر انگشت"),
        ("Select a peer", "یک همتا را انتخاب کنید"),
        ("Select peers", "همتایان را انتخاب کنید"),
        ("Plugins", "پلاگین ها"),
        ("Uninstall", "حذف نصب"),
        ("Update", "به روز رسانی"),
        ("Enable", "فعال کردن"),
        ("Disable", "غیر فعال کردن"),
        ("Options", "گزینه ها"),
        ("resolution_original_tip", "وضوح اصلی"),
        ("resolution_fit_local_tip", "متناسب با وضوح محلی"),
        ("resolution_custom_tip", "وضوح سفارشی"),
        ("Collapse toolbar", "جمع کردن نوار ابزار"),
        ("Accept and Elevate", "بپذیرید و افزایش دهید"),
        ("accept_and_elevate_btn_tooltip", "را افزایش دهید UAC اتصال را بپذیرید و مجوزهای."),
        ("clipboard_wait_response_timeout_tip", "زمان انتظار برای مشخص شدن وضعیت کپی تمام شد."),
        ("Incoming connection", "اتصال ورودی"),
        ("Outgoing connection", "اتصال خروجی"),
        ("Exit", "خروج"),
        ("Open", "باز کردن"),
        ("logout_tip", "آیا برای خارج شدن مطمئن هستید؟"),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_StarDesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
    ].iter().cloned().collect();
}
